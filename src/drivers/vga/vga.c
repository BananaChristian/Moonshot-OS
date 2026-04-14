#include "vga.h"

static uint16_t *vga = (uint16_t *)VGA_ADDRESS;
static uint8_t cursor_row = 0;
static uint8_t cursor_col = 0;
static uint8_t current_color = VGA_COLOR(VGA_WHITE, VGA_BLACK);

static void vga_putchar(char c, uint8_t color) {
  if (c == '\n') {
    cursor_col = 0;
    cursor_row++;
    return;
  }

  if (cursor_col >= VGA_WIDTH) {
    cursor_col = 0;
    cursor_row++;
  }

  if (cursor_row >= VGA_HEIGHT) {
    cursor_row = 0;
  }

  uint16_t index = cursor_row * VGA_WIDTH + cursor_col;
  vga[index] = (uint16_t)c | ((uint16_t)color << 8);
  cursor_col++;
}

void vga_init(void) {
  for (uint16_t i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++) {
    vga[i] = (uint16_t)' ' | ((uint16_t)current_color << 8);
  }

  cursor_row = 0;
  cursor_col = 0;
}

void vga_print(const char *str) {
  for (uint16_t i = 0; str[i] != '\0'; i++) {
    vga_putchar(str[i], current_color);
  }
}

void vga_println(const char *str) {
  vga_print(str);
  vga_putchar('\n', current_color);
}

void vga_print_color(const char *str, uint8_t color) {
  for (uint16_t i = 0; str[i] != '\0'; i++) {
    vga_putchar(str[i], color);
  }
}
