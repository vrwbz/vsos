#![no_std]
#![no_main]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    let message = b"VSOS - kernel online";

    for (index, byte) in message.iter().enumerate() {
        unsafe {
            vga_buffer.add(index * 2).write_volatile(*byte);
            vga_buffer.add(index * 2 + 1).write_volatile(0x0f);
        }
    }

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
