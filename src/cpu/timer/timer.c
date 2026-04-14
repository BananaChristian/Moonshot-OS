#include "timer.h"
#include "cpu/io.h"
#include "cpu/irq/irq.h"
#include "cpu/pic/pic.h"

#define PIT_CHANNEL0 0x40
#define PIT_COMMAND 0x43

static uint64_t ticks = 0;

static void timer_handler(void) { ticks++; }

void timer_init(uint32_t frequency) {
  uint32_t divisor = 1193182 / frequency;

  outb(PIT_COMMAND, 0x36);
  outb(PIT_CHANNEL0, divisor & 0xFF);
  outb(PIT_CHANNEL0, (divisor >> 8) & 0xFF);

  irq_register(0, timer_handler);
  pic_unmask(0);
}

uint64_t timer_get_ticks(void) { return ticks; }
