#include "irq.h"
#include "cpu/idt/idt.h"
#include "cpu/pic/pic.h"

extern void irq0(void);
extern void irq1(void);
extern void irq2(void);
extern void irq3(void);
extern void irq4(void);
extern void irq5(void);
extern void irq6(void);
extern void irq7(void);
extern void irq8(void);
extern void irq9(void);
extern void irq10(void);
extern void irq11(void);
extern void irq12(void);
extern void irq13(void);
extern void irq14(void);
extern void irq15(void);

static irq_handler_t irq_handlers[16] = {0};

void irq_init(void) {
  idt_set_entry(32, (uint64_t)irq0, 0x8E);
  idt_set_entry(33, (uint64_t)irq1, 0x8E);
  idt_set_entry(34, (uint64_t)irq2, 0x8E);
  idt_set_entry(35, (uint64_t)irq3, 0x8E);
  idt_set_entry(36, (uint64_t)irq4, 0x8E);
  idt_set_entry(37, (uint64_t)irq5, 0x8E);
  idt_set_entry(38, (uint64_t)irq6, 0x8E);
  idt_set_entry(39, (uint64_t)irq7, 0x8E);
  idt_set_entry(40, (uint64_t)irq8, 0x8E);
  idt_set_entry(41, (uint64_t)irq9, 0x8E);
  idt_set_entry(42, (uint64_t)irq10, 0x8E);
  idt_set_entry(43, (uint64_t)irq11, 0x8E);
  idt_set_entry(44, (uint64_t)irq12, 0x8E);
  idt_set_entry(45, (uint64_t)irq13, 0x8E);
  idt_set_entry(46, (uint64_t)irq14, 0x8E);
  idt_set_entry(47, (uint64_t)irq15, 0x8E);
}

void irq_register(uint8_t irq, irq_handler_t handler) {
  irq_handlers[irq] = handler;
}

void irq_handler_c(void) { pic_send_eoi(0); }
