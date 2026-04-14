#include "idt.h"
#include "cpu/isr/isr.h"
#include <stdint.h>

#define IDT_ENTRIES 256
#define KERNEL_CS 0x08

static idt_entry_t idt[IDT_ENTRIES];
static idt_descriptor_t idt_descriptor;

void idt_set_entry(uint8_t n, uint64_t handler, uint8_t flags) {
  idt[n].low = handler & 0xFFFF;
  idt[n].selector = KERNEL_CS;
  idt[n].ist = 0;
  idt[n].flags = flags;
  idt[n].mid = (handler >> 16) & 0xFFFF;
  idt[n].high = (handler >> 32) & 0xFFFFFFFF;
  idt[n].reserved = 0;
}

void idt_init(void) {
  idt_descriptor.limit = (sizeof(idt_entry_t) * IDT_ENTRIES) - 1;
  idt_descriptor.base = (uint64_t)idt;

  for (int i = 0; i < IDT_ENTRIES; i++) {
    idt_set_entry(i, (uint64_t)isr_stub, 0x8E);
  }

  idt_set_entry(0, (uint64_t)isr0, 0x8E);
  idt_set_entry(1, (uint64_t)isr1, 0x8E);
  idt_set_entry(2, (uint64_t)isr2, 0x8E);
  idt_set_entry(3, (uint64_t)isr3, 0x8E);
  idt_set_entry(4, (uint64_t)isr4, 0x8E);
  idt_set_entry(5, (uint64_t)isr5, 0x8E);
  idt_set_entry(6, (uint64_t)isr6, 0x8E);
  idt_set_entry(7, (uint64_t)isr7, 0x8E);
  idt_set_entry(8,(uint64_t)isr8,0x8E);
  idt_set_entry(10,(uint64_t)isr10,0x8E);
  idt_set_entry(11,(uint64_t)isr11,0x8E);
  idt_set_entry(12,(uint64_t)isr12,0x8E);
  idt_set_entry(13,(uint64_t)isr13,0x8E);
  idt_set_entry(14,(uint64_t)isr14,0x8E);
  idt_set_entry(16,(uint64_t)isr16,0x8E);
  idt_set_entry(17,(uint64_t)isr17,0x8E);
  idt_set_entry(18,(uint64_t)isr18,0x8E);
  idt_set_entry(19,(uint64_t)isr19,0x8E);
  idt_set_entry(20,(uint64_t)isr20,0x8E);

  __asm__ volatile ("lidt %0": : "m"(idt_descriptor));
}
