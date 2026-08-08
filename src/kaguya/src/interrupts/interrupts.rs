use crate::println;
use spin::Lazy;
use x86_64::instructions::port::Port;
use x86_64::registers::control::Cr2;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

// Interrupt Vector Offset for PIC
pub const PIC_1_OFFSET: u8 = 32;

// Safe, thread-safe IDT loaded at runtime
static IDT: Lazy<InterruptDescriptorTable> = Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    // CPU Exceptions
    idt.divide_error.set_handler_fn(divide_by_zero_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);

    // Hardware Interrupts (Vector 32 / 0x20 = Timer)
    idt[PIC_1_OFFSET as usize].set_handler_fn(timer_interrupt_handler);

    idt
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
    // Send End-of-Interrupt (EOI) signal to PIC Master (Port 0x20)
    unsafe {
        let mut pic_master_cmd: Port<u8> = Port::new(0x20);
        pic_master_cmd.write(0x20);
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
        master_data.write(0xFE);
        slave_data.write(0xFF);
    }
}
