use core::ptr::{read_volatile, write_volatile};

#[allow(dead_code)]
#[repr(C)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

//This is the combination of what sits in a grid
#[repr(C)]
pub struct ScreenChar {
    ascii_character: u8,
    color_code: u8,
}

pub struct VGA {
    width: u8,
    cursor: usize,  //Marks the current position
    color_code: u8, //Track the current color
    buffer: *mut ScreenChar,
}

impl VGA {
    pub fn new(foreground: Color, background: Color) -> Self {
        VGA {
            width: 80,
            cursor: 0, //Start at 0
            color_code: (background as u8) << 4 | (foreground as u8),
            buffer: 0xb8000 as *mut ScreenChar,
        }
    }

    pub fn clear(&self) {
        for i in 0..2000 {
            unsafe {
                write_volatile(
                    self.buffer.offset(i),
                    ScreenChar {
                        ascii_character: b' ',
                        color_code: self.color_code,
                    },
                );
            }
        }
    }

    fn write_char(&mut self, char: u8) {
        if char == b'\n' {
            self.cursor = ((self.cursor / 80) + 1) * 80;
            self.check_scroll();
            return;
        }

        unsafe {
            write_volatile(
                self.buffer.offset(self.cursor as isize),
                ScreenChar {
                    ascii_character: char,
                    color_code: self.color_code,
                },
            );
        }
        self.cursor += 1;
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write_char(byte);
        }
    }

    fn check_scroll(&mut self) {
        if self.cursor < 2000 {
            return;
        }

        unsafe {
            //Shift the upper shleves up by one
            for i in 0..1920 {
                let next_line_char = read_volatile(self.buffer.offset(i + self.width as isize));
                write_volatile(self.buffer.offset(i), next_line_char);
            }

            //Clear out the last row to create space
            for i in 1920..2000 {
                write_volatile(
                    self.buffer.offset(i),
                    ScreenChar {
                        ascii_character: b' ',
                        color_code: self.color_code,
                    },
                );
            }
        }

        self.cursor = 1920;
    }
}
