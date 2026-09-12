#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let mut screen = Screen::new();
    screen.draw();

    loop {
        x86_64::instructions::hlt();
    }
}

struct Screen {
    buffer: *mut u8,
}

impl Screen {
    const WIDTH: usize = 80;
    const HEIGHT: usize = 25;

    fn new() -> Self {
        Self { buffer: 0xb8000 as *mut u8 }
    }

    fn draw(&mut self) {
        self.fill_rect(0, 0, Self::WIDTH, Self::HEIGHT, 0x10);
        self.fill_rect(0, 0, Self::WIDTH, 3, 0x1f);
        self.text(2, 1, "VSOS", 0x1f);
        self.text(11, 1, "VIRTUAL SYSTEM OPERATING SYSTEM", 0x1f);
        self.text(66, 1, "ONLINE", 0x1a);

        self.text(2, 4, "SYSTEM OVERVIEW", 0x0b);
        self.draw_box(2, 5, 36, 8, 0x08);
        self.text(4, 7, "KERNEL", 0x07);
        self.text(25, 7, "VSOS 0.1.0", 0x0f);
        self.text(4, 9, "ARCHITECTURE", 0x07);
        self.text(25, 9, "x86_64", 0x0f);
        self.text(4, 11, "RUNTIME", 0x07);
        self.text(25, 11, "BARE METAL", 0x0f);

        self.draw_box(41, 5, 37, 8, 0x08);
        self.text(43, 7, "BOOT STATUS", 0x07);
        self.text(69, 7, "READY", 0x1a);
        self.text(43, 9, "DISPLAY", 0x07);
        self.text(69, 9, "VGA TEXT", 0x0f);
        self.text(43, 11, "UPTIME", 0x07);
        self.text(69, 11, "00:00:01", 0x0f);

        self.text(2, 15, "BOOT SERVICES", 0x0b);
        self.draw_box(2, 16, 76, 6, 0x08);
        self.status(4, 18, "MEMORY MAP", "READY");
        self.status(29, 18, "INTERRUPTS", "STANDBY");
        self.status(56, 18, "SHELL", "NEXT");
        self.text(2, 24, "  VSOS  /  KERNEL ONLINE  /  BUILD 2026.09", 0x17);
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
        for (offset, byte) in value.bytes().enumerate() {
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

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
