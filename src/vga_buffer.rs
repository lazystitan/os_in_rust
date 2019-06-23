use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

//可用来打印至vga text buffer的全局实体
//默认不能在静态变？常量中使用非静态函数，导入lazy_static允许这么做
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

#[allow(dead_code)] //关闭rust对未被使用的代码存在的警告
#[derive(Debug, Clone, Copy, Eq, PartialEq)] //可以比较，复制，打印
#[repr(u8)] //每个颜色都存储为u8
pub enum Color {
    //颜色
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

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(transparent)] //保证ColorCode和u8有一样的data layout（数据行为）
struct ColorCode(u8);
//完整的foreground和background的颜色

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(C)] //rust中默认没有保证实际域的顺序和代码一样，因此启用c的结构体来保证
struct ScreenChar {
    //字符+颜色
    ascii_character: u8,
    color_code: ColorCode,
}

//vag buffer的常数
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//vga buffer
#[repr(transparent)]
struct Buffer {
    //volatile易变的，具有side effect的
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    //写入buffer的结构体
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                //可打印的字符
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //其他■
                _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(), //换行
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    //若此行已满，换行
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
