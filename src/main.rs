#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let mut screen = Screen::new();
    screen.draw();
    let mut keyboard = Keyboard::new();

    loop {
        if let Some(scancode) = keyboard.read_scancode() {
            if let Some(key) = Keyboard::ascii(scancode) {
                screen.handle_key(key);
            }
        }
        x86_64::instructions::hlt();
    }
}

struct Screen {
    buffer: *mut u8,
    input: [u8; 32],
    input_len: usize,
}

impl Screen {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    fn new() -> Self {
        Self {
            buffer: 0xb8000 as *mut u8,
            input: [0; 32],
            input_len: 0,
        }
    }

    fn draw(&mut self) {
        self.fill_rect(0, 0, Self::WIDTH, Self::HEIGHT, 0x10);
        self.fill_rect(0, 0, Self::WIDTH, 3, 0x1f);
        self.text(2, 1, "VSOS", 0x1f);
        self.text(11, 1, "VIRTUAL SYSTEM OPERATING SYSTEM", 0x1f);
        self.text(66, 1, "[ONLINE]", 0x1a);

        self.text(2, 4, "SYSTEM OVERVIEW", 0x0b);
        self.text(56, 4, "BUILD 0.1.0", 0x08);
        self.draw_box(2, 5, 36, 9, 0x08);
        self.text(4, 6, "KERNEL", 0x07);
        self.text(25, 6, "VSOS 0.1.0", 0x0f);
        self.text(4, 8, "ARCHITECTURE", 0x07);
        self.text(25, 8, "x86_64", 0x0f);
        self.text(4, 10, "RUNTIME", 0x07);
        self.text(25, 10, "BARE METAL", 0x0f);
        self.text(4, 12, "SECURITY", 0x07);
        self.text(25, 12, "RING 0", 0x0f);

        self.draw_box(41, 5, 37, 9, 0x08);
        self.text(43, 6, "BOOT STATUS", 0x07);
        self.text(69, 6, "READY", 0x1a);
        self.text(43, 8, "DISPLAY", 0x07);
        self.text(69, 8, "VGA TEXT", 0x0f);
        self.text(43, 10, "UPTIME", 0x07);
        self.text(69, 10, "RUNNING", 0x0f);
        self.text(43, 12, "INPUT", 0x07);
        self.text(69, 12, "PS/2", 0x0f);

        self.text(2, 15, "BOOT SERVICES", 0x0b);
        self.draw_box(2, 16, 76, 6, 0x08);
        self.status(4, 18, "MEMORY MAP", "READY");
        self.status(29, 18, "INTERRUPTS", "STANDBY");
        self.status(56, 18, "SHELL", "NEXT");
        self.text(2, 22, "TYPE HELP FOR COMMANDS", 0x08);
        self.text(2, 23, "COMMAND > ", 0x0b);
        self.text_bytes(12, 23, &self.input[..self.input_len], 0x0f);
        self.cell(12 + self.input_len, 23, b'_', 0x0f);
        self.text(2, 24, "  VSOS  /  KERNEL ONLINE  /  BUILD 2026.09", 0x17);
    }

    fn handle_key(&mut self, key: u8) {
        match key {
            b'\n' => self.submit_command(),
            8 => {
                if self.input_len > 0 {
                    self.input_len -= 1;
                    self.input[self.input_len] = 0;
                    self.cell(12 + self.input_len, 23, b'_', 0x0f);
                }
            }
            byte if self.input_len < self.input.len() && byte.is_ascii_graphic() => {
                self.input[self.input_len] = byte;
                self.input_len += 1;
                self.cell(11 + self.input_len, 23, byte, 0x0f);
                self.cell(12 + self.input_len, 23, b'_', 0x0f);
            }
            _ => {}
        }
    }

    fn submit_command(&mut self) {
        self.clear_line(22);
        if self.input_len == 0 {
            self.text(2, 22, "ENTER A COMMAND - TRY HELP", 0x0e);
        } else if self.matches(b"help") {
            self.text(2, 22, "HELP: STATUS  ABOUT  VERSION  CLEAR  HELP", 0x0b);
        } else if self.matches(b"status") {
            self.text(
                2,
                22,
                "STATUS: KERNEL READY / VGA READY / INPUT READY",
                0x1a,
            );
        } else if self.matches(b"about") {
            self.text(
                2,
                22,
                "VSOS: A SMALL, CURIOUS SYSTEM BUILT FROM FIRST PRINCIPLES",
                0x0f,
            );
        } else if self.matches(b"version") {
            self.text(2, 22, "VSOS KERNEL 0.1.0 / x86_64 / NIGHTLY RUST", 0x0f);
        } else if self.matches(b"clear") {
            self.input = [0; 32];
            self.input_len = 0;
            self.draw();
            return;
        } else {
            self.text(2, 22, "UNKNOWN COMMAND - TRY HELP", 0x0c);
        }
        self.input = [0; 32];
        self.input_len = 0;
        self.clear_line(23);
        self.text(2, 23, "COMMAND > ", 0x0b);
        self.cell(12, 23, b'_', 0x0f);
    }

    fn matches(&self, command: &[u8]) -> bool {
        self.input_len == command.len() && self.input[..self.input_len] == *command
    }

    fn clear_line(&self, row: usize) {
        self.fill_rect(0, row, Self::WIDTH, 1, 0x10);
    }

    fn fill_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u8) {
        for row in y..(y + height) {
            for column in x..(x + width) {
                self.cell(column, row, b' ', color);
            }
        }
    }

    fn draw_box(&self, x: usize, y: usize, width: usize, height: usize, color: u8) {
        for column in x..(x + width) {
            self.cell(column, y, b'-', color);
            self.cell(column, y + height - 1, b'-', color);
        }
        for row in y..(y + height) {
            self.cell(x, row, b'|', color);
            self.cell(x + width - 1, row, b'|', color);
        }
        self.cell(x, y, b'+', color);
        self.cell(x + width - 1, y, b'+', color);
        self.cell(x, y + height - 1, b'+', color);
        self.cell(x + width - 1, y + height - 1, b'+', color);
    }

    fn status(&self, x: usize, y: usize, label: &str, value: &str) {
        self.cell(x, y, b'>', 0x0b);
        self.text(x + 2, y, label, 0x0f);
        self.text(x + 2, y + 1, value, 0x1a);
    }

    fn text(&self, x: usize, y: usize, value: &str, color: u8) {
        self.text_bytes(x, y, value.as_bytes(), color);
    }

    fn text_bytes(&self, x: usize, y: usize, value: &[u8], color: u8) {
        for (offset, byte) in value.iter().copied().enumerate() {
            if x + offset >= Self::WIDTH || y >= Self::HEIGHT {
                break;
            }
            self.cell(x + offset, y, byte, color);
        }
    }

    fn cell(&self, x: usize, y: usize, character: u8, color: u8) {
        if x >= Self::WIDTH || y >= Self::HEIGHT {
            return;
        }
        let offset = (y * Self::WIDTH + x) * 2;
        unsafe {
            self.buffer.add(offset).write_volatile(character);
            self.buffer.add(offset + 1).write_volatile(color);
        }
    }
}

struct Keyboard {
    status: Port<u8>,
    data: Port<u8>,
}

impl Keyboard {
    fn new() -> Self {
        Self {
            status: Port::new(0x64),
            data: Port::new(0x60),
        }
    }

    fn read_scancode(&mut self) -> Option<u8> {
        unsafe {
            if self.status.read() & 1 == 0 {
                None
            } else {
                Some(self.data.read())
            }
        }
    }

    fn ascii(scancode: u8) -> Option<u8> {
        const KEYS: &[u8] = b"?1234567890-=\tqwertyuiop[]\n?asdfghjkl;'`?\\zxcvbnm,./";
        if scancode & 0x80 != 0 {
            return None;
        }
        match scancode {
            0x0e => Some(8),
            0x39 => Some(b' '),
            0x1c => Some(b'\n'),
            code if (code as usize) < KEYS.len() => Some(KEYS[code as usize]),
            _ => None,
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
