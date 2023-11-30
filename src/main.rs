#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod kernelc;
mod multiboot;
mod o1heap;
mod terminal;
mod vga;

use alloc::vec;

use crate::{allocator::ALLOC, multiboot::MultibootInfo};

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("boot.S"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{info}");
    loop {}
}

#[no_mangle]
pub extern "C" fn init(multiboot_magic: u32, info: &MultibootInfo) -> ! {
    assert!(multiboot_magic == 0x2BADB002);
    unsafe { ALLOC.init(info) }

    let bootloader_name = info.get_bootloader_name();
    println!("Booted from bootloader `{bootloader_name}`");

    let mut vec = vec![1, 2, 3, 4, 5, 6];
    println!("{vec:?}");
    for _ in 0..vec.len() {
        println!("Popped number: {}", vec.pop().unwrap());
    }
    drop(vec);

    loop {}
}
