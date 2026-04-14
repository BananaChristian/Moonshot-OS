#ifndef PIC_H
#define PIC_H

#include <stdint.h>

#define PIC1_COMMAND 0x20 //Master PIC command port
#define PIC1_DATA 0x21 //Master PIC data port
#define PIC2_COMMAND 0xA0 //Slave PIC command port
#define PIC2_DATA 0xA1 //Slave PIC data port

#define PIC_EOI 0x20 //End of interupt

#define PIC1_OFFSET 0x20
#define PIC2_OFFSET 0x28

void pic_init(void);
void pic_send_eoi(uint8_t irq);
void pic_mask(uint8_t irq);
void pic_unmask(uint8_t irq);

#endif 
