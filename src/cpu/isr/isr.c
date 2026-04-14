#include "isr.h"
#include "drivers/vga/vga.h"

static const char *exception_messages[]={
    "Division by Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Overflow",
    "Bound Range Exceeded",
    "Invalid Opcode",
    "Device not Available",
    "Double Fault",
    "Reserved",
    "Invalid TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Reserved",
    "Floating Point Exception",
    "Alignment Check",
    "Machine Check",
    "SIMD Floating Point Exception",
    "Virtualization Exception",
};

void isr_handler_c(void){
    vga_print_color("[ FAULT ]", VGA_COLOR(VGA_RED,VGA_RED));
    vga_println("CPU Exception fired");
    while(1){}
}
