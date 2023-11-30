use crate::vga;
use core::fmt::Write;

pub static TERMINAL: Terminal = Terminal::new();

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        #[allow(unused_unsafe)]
        unsafe {
            use $crate::terminal::Terminal;
            use core::fmt::Write as FmtWrite;
            let writer = &$crate::terminal::TERMINAL as *const Terminal;
            // write_fmt needs writer as &mut, but we only access it as *const. Cast to fulfill the
            // API requirements
            let writer = writer as *mut Terminal;
            if !(*writer).initialized {
                (*writer).init();
            }
            write!(&mut *(writer), $($arg)*).expect("Failed to print")
        }
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        print!($($arg)*);
        print!("\n");
    }
}

pub struct Terminal {
    pub initialized: bool,
    row: usize,
    column: usize,
    color: u8,
    buffer: *mut u16,
}

impl Terminal {
    pub const fn new() -> Self {
        let row = 0;
        let column = 0;
        let color = vga::entry_color(vga::Color::LightGray, vga::Color::Black);
        let buffer = 0xB8000 as *mut u16;

        Self {
            initialized: false,
            row,
            column,
            color,
            buffer,
        }
    }

    pub fn init(&mut self) {
        for y in 0..vga::HEIGHT {
            for x in 0..vga::WIDTH {
                let index = y * vga::WIDTH + x;
                unsafe {
                    *self.buffer.add(index) = vga::entry(b' ', self.color);
                }
            }
        }
        self.initialized = true;
    }

    #[allow(dead_code)]
    pub fn set_color(&mut self, color: u8) {
        self.color = color;
    }

    fn put_entry_at(&mut self, c: u8, color: u8, x: usize, y: usize) {
        let index = y * vga::WIDTH + x;
        unsafe {
            *self.buffer.add(index) = vga::entry(c, color);
        }
    }

    pub fn put_char(&mut self, c: u8) {
        if c == b'\n' {
            self.row += 1;
            self.column = 0;
            return;
        }

        self.put_entry_at(c, self.color, self.column, self.row);
        self.column += 1;
        if self.column == vga::WIDTH {
            self.column = 0;
            self.row += 1;
            if self.row == vga::HEIGHT {
                self.row = 0;
            }
        }
    }

    pub fn write(&mut self, data: &[u8]) {
        for c in data {
            self.put_char(*c);
        }
    }
}

impl Write for Terminal {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s.as_bytes());
        Ok(())
    }
}

unsafe impl Sync for Terminal {}
