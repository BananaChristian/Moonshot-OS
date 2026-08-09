use crate::gdt;
use crate::{print, println};
use pc_keyboard::{DecodedKey, HandleControl, Keyboard, ScancodeSet1, layouts};
use spin::Lazy;
use spin::mutex::Mutex;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// Interrupt Vector Offset for PIC
pub const PIC_1_OFFSET: u8 = 32;
pub const TIMER_INTERRUPT_INDEX: u8 = PIC_1_OFFSET;
pub const KEYBOARD_INTERRUPT_INDEX: u8 = PIC_1_OFFSET + 1;
pub const SPURIOUS_INTERRUPT_INDEX: u8 = PIC_1_OFFSET + 7;

// Safe, thread-safe IDT loaded at runtime
static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    // CPU Exception handlers
    idt.divide_error.set_handler_fn(divide_by_zero_handler);
    
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt.page_fault.set_handler_fn(page_fault_handler);

    // Hardware Interrupts (Vector 32 / 0x20 = Timer)
    idt[TIMER_INTERRUPT_INDEX as usize].set_handler_fn(timer_interrupt_handler);
    idt[KEYBOARD_INTERRUPT_INDEX as usize].set_handler_fn(keyboard_interrupt_handler);
    idt[SPURIOUS_INTERRUPT_INDEX as usize].set_handler_fn(spurious_interrupt_handler);

    idt
});

static KEYBOARD: Lazy<Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>>> = Lazy::new(|| {
    Mutex::new(Keyboard::new(
        ScancodeSet1::new(),
        layouts::Us104Key,
        HandleControl::MapLettersToUnicode,
    ))
});

pub fn initialize_interrupts() {
    // Load IDT (Automatically reads active CS register)
    IDT.load();

    //  Remap 8259 PIC
    remap_pic();

    // 3. Enable Interrupts (`sti`)
    x86_64::instructions::interrupts::enable();
}

//Exception Handlers
extern "x86-interrupt" fn divide_by_zero_handler(stack_frame: InterruptStackFrame) {
    println!("[EXCEPTION] DIVIDE BY ZERO\n{:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    println!(
        "[EXCEPTION] DOUBLE FAULT (Code: {})\n{:#?}",
        error_code, stack_frame
    );
    loop {
        x86_64::instructions::hlt();
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println!("[EXCEPTION] PAGE FAULT");
    println!("  Accessed Address: {:?}", Cr2::read());
    println!("  Error Code      : {:?}", error_code);
    println!("  Frame           : {:#?}", stack_frame);
    loop {
        x86_64::instructions::hlt();
    }
}

//Hardware Interrupts
extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    send_eoi(0);
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port: Port<u8> = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    let mut keyboard = KEYBOARD.lock();

    // Process raw scancodes through the state machine
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(raw_key) => print!("{:?}", raw_key),
            }
        }
    }

    send_eoi(1);
}

extern "x86-interrupt" fn spurious_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Real spurious IRQ 7 interrupts on hardware/QEMU do NOT need an EOI sent to Master PIC.
    // We simply catch and ignore them so the kernel doesn't fault.
}

pub fn send_eoi(irq: u8) {
    if irq >= 8 {
        let mut slave_cmd: Port<u8> = Port::new(0xA0);
        unsafe {
            slave_cmd.write(0x20);
        }
    }
    let mut master_cmd: Port<u8> = Port::new(0x20);
    unsafe {
        master_cmd.write(0x20);
    }
}

fn remap_pic() {
    let mut master_cmd: Port<u8> = Port::new(0x20);
    let mut master_data: Port<u8> = Port::new(0x21);
    let mut slave_cmd: Port<u8> = Port::new(0xA0);
    let mut slave_data: Port<u8> = Port::new(0xA1);
    let mut wait_port: Port<u8> = Port::new(0x80);

    unsafe {
        let mut io_wait = || wait_port.write(0);

        // ICW1: Initialize Master & Slave
        master_cmd.write(0x11);
        io_wait();
        slave_cmd.write(0x11);
        io_wait();

        // ICW2: Vector offsets (Master = 32, Slave = 40)
        master_data.write(PIC_1_OFFSET);
        io_wait();
        slave_data.write(PIC_1_OFFSET + 8);
        io_wait();

        // ICW3: Cascading setup
        master_data.write(0x04);
        io_wait();
        slave_data.write(0x02);
        io_wait();

        // ICW4: 8086 mode
        master_data.write(0x01);
        io_wait();
        slave_data.write(0x01);
        io_wait();

        // Mask ports: Unmask IRQ0 (Timer) on Master, mask all on Slave
        master_data.write(0xFC);
        slave_data.write(0xFF);
    }
}
