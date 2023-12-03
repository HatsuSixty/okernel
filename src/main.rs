#![no_std]
#![no_main]

extern crate alloc;

mod allocator;
mod cpu;
mod drivers;
mod kernelc;
mod multiboot;
mod o1heap;
mod terminal;
mod vga;

use crate::{allocator::ALLOC, drivers::disk::{MbrPartition, open_disk, get_bytes}, multiboot::MultibootInfo};

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

    let mut mbrpartition = MbrPartition::default();
    open_disk(0, &mut mbrpartition as *mut MbrPartition);

    println!("MBR: {mbrpartition:?}\n");

    let data: &mut [u8; 512] = &mut [0; 512];
    get_bytes(data, 1, 1);

    print!("First sector after MBR: ");
    let mut last: i32 = -1;
    for (i, x) in data.iter().enumerate() {
        if i == 0 {
            print!("[");
        }
        if *x as i32 == last {
            print!(".");
        } else {
            print!("{x:#02X}");
        }
        if i == data.len()-1 {
            println!("]");
        } else {
            print!(", ")
        }
        last = *x as i32;
    }

    loop {}
}
