#![no_std]
#![no_main]

use core::panic::PanicInfo;

use crate::vga::{Color, VGA};

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kmain() -> ! {
    let mut vga = VGA::new(Color::LightCyan, Color::Black);
    vga.clear();

    vga.write_string("[OK] Kaguya kernel core initialized.\n");
    vga.write_string("[OK] VGA text mode initialized.\n");

    loop {}
}
