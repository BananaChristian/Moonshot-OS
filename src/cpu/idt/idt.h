#ifndef IDT_H
#define IDT_H

#include <stdint.h>

typedef struct {
    uint16_t low;       
    uint16_t selector; 
    uint8_t  ist;       
    uint8_t  flags;     
    uint16_t mid;       
    uint32_t high;      
    uint32_t reserved;  
} __attribute__((packed)) idt_entry_t;

typedef struct {
    uint16_t limit;
    uint64_t base;
} __attribute__((packed)) idt_descriptor_t;

void idt_init(void);
void idt_set_entry(uint8_t n, uint64_t handler, uint8_t flags);

#endif
