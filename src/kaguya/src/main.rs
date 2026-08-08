#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

use crate::interrupts::initialize_interrupts;

mod interrupts;
mod vga;

entry_point!(kmain);

fn kmain(_boot_info: &'static BootInfo) -> ! {
    // Clear VGA screen first to prove memory access works
    vga::VGA_WRITER.lock().clear();

    println!("[OK] Kaguya core initialized.");
    println!("[OK] VGA text mode initialized.");

    // Initialize IDT and hardware interrupts
    initialize_interrupts();
    println!("[OK] IDT & PIC initialized.");

    // Sleep state
    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}
