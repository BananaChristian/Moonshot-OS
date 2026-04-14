#ifndef IRQ_H
#define IRQ_H

#include <stdint.h>

typedef struct{
    uint64_t r15,r14,r13,r12,r11,r10,r9,r8;
    uint64_t rbp,rdi,rsi,rdx,rcx,rbx,rax;
    uint64_t int_no,error_code;
    uint64_t rip,cs,rflags, rsp, ss;
}registers_t;

typedef void (*irq_handler_t)(void);

void irq_init(void);
void irq_register(uint8_t irq,irq_handler_t handler);
void irq_handler_c(void);

#endif
