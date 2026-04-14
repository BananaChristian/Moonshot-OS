#include "cpu/idt/idt.h"
#include "cpu/irq/irq.h"
#include "cpu/pic/pic.h"
#include "cpu/timer/timer.h"
#include "drivers/vga/vga.h"

void kernel_main(uint32_t magic, void *multiboot_info) {
  vga_init();

  vga_println("Moonshot OS");
  vga_println("Kaguya Kernel v0.1");
  vga_println("---------------------");

  idt_init();
  vga_print_color("[ OK ] ", VGA_COLOR(VGA_GREEN, VGA_BLACK));
  vga_println("Interrupt Descriptor Table");

  pic_init();
  vga_print_color("[ OK ] ", VGA_COLOR(VGA_GREEN, VGA_BLACK));
  vga_println("Programmable Interrupt Controller");

  irq_init();
  vga_print_color("[ OK ] ", VGA_COLOR(VGA_GREEN, VGA_BLACK));
  vga_println("IRQ Handlers");

  __asm__ volatile("sti");

  timer_init(100);
  vga_print_color("[ OK ] ", VGA_COLOR(VGA_GREEN, VGA_BLACK));
  vga_println("Timer (100Hz)");

  vga_print_color("[ OK ] ", VGA_COLOR(VGA_GREEN, VGA_BLACK));
  vga_println("VGA driver");

  while (1) {
  }
}
