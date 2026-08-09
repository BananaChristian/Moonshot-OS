use spin::Lazy;
use x86_64::{
    VirtAddr,
    structures::{
        gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector},
        tss::TaskStateSegment,
    },
};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Dedicated 4KB stack array for Double Fault handling
static mut DOUBLE_FAULT_STACK: [u8; 4096] = [0; 4096];

// Configure Task State Segment 
static TSS: Lazy<TaskStateSegment> = Lazy::new(|| {
    let mut tss = TaskStateSegment::new();

    // Calculate the top of our static double fault stack (stacks grow down on x86)
    let stack_start = VirtAddr::from_ptr(&raw const DOUBLE_FAULT_STACK);
    let stack_end = stack_start + 4096usize;

    // Assign the top of the stack to IST index 0
    tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = stack_end;

    tss
});

// Container struct to hold selectors so we can reload CS and TSS registers safely
pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

// Build Global Descriptor Table (GDT)
static GDT: Lazy<(GlobalDescriptorTable, Selectors)> = Lazy::new(|| {
    let mut gdt = GlobalDescriptorTable::new();

    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));

    (
        gdt,
        Selectors {
            code_selector,
            tss_selector,
        },
    )
});

pub fn init() {
    use x86_64::instructions::segmentation::{CS, Segment};
    use x86_64::instructions::tables::load_tss;

    // Load GDT table
    GDT.0.load();

    unsafe {
        // Reload Code Segment register (CS) with our new kernel code selector
        CS::set_reg(GDT.1.code_selector);

        // Load Task State Segment selector into the CPU TR register
        load_tss(GDT.1.tss_selector);
    }
}
