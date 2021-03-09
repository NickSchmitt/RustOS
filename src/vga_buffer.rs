use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;

// using the lazy_static! macro to lazily initialize a static at runtime.
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer { //using a spinlock in lieu of mutex
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// specifying the colors of the vga buffer
#[allow(dead_code)] // disable warning for unused enum variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)] //enable copy semantics and make it printable and comparable
#[repr(u8)] // store each enum variant as a u8
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)] // ensure ColorCode has exact data layout of an u8
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)] //Default struct field ordering is undefined in Rust. repr(C) guarantees correct field ordering by laying them out like C structs.
struct ScreenChar {
    ascii_character:u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;


// The compiler doesn't know that we really access VGA buffer memory (instead of normal RAM) and knows nothing about the side effect that some characters appear on the screen. It might see these writes as unnecessary and omit them. Using Volatile<> tells the compiler that the write has side effects and should not be optimized away. This ensures that we can't accidentally write to it through a “normal” write. Instead, we have to use the write method now.
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// will always write to the last line and shift lines up. 
pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer, //`'static` to specify reference is valid for the whole program (which is true for the VGHA te)
}

// use the Writer to modify the buffer's characters. Method to write a single ASCII byte

impl Writer {
    pub fn write_byte(&mut self, byte: u8){
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
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

    // Write whole strings

    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }
    // move every char one line up, delete the top line, start at the beginning of the last line
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT{
            for col in 0..BUFFER_WIDTH{
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row-1][col].write(character)
            }
        }
        self.clear_row(BUFFER_HEIGHT -1);
        self.column_position = 0;
    }

    // clear a row by overwriting it with space characters.
    fn clear_row(&mut self, row: usize){
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

// Implement core::fmt::Write to support Rust's formatting macros
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

// ***PRINTLN MACRO STUFF***
#[macro_export] //makes the macro available to the whole crate and external crates. Also places the macro at the crate root 
macro_rules! print {
    //$crate ensures macro works outisde std by expanding to std when used in other crates
    // foramt_args! builds a fmt::Arguments type from the args
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*))); 
}

#[macro_export]
macro_rules! println {
    // prefix the print! invocation with $crate so we don't have to import print! if we just want to use println
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// private implementation detail, so hide it from the docs with doc(hidden)
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(||{
        WRITER.lock().write_fmt(args).unwrap();
    });
}
// ***END PRINTLN MACRO STUFF***

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output(){
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for(i, c) in s.chars().enumerate() {
        let screen_char=WRITER.lock().buffer.chars[BUFFER_HEIGHT -2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}