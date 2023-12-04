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

use crate::{
    allocator::ALLOC,
    drivers::{
        ata::DiskType,
        disk::{get_bytes, open_disk, MbrPartition},
    },
    multiboot::MultibootInfo,
};

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
    println!("[INI] Booted from bootloader `{bootloader_name}`");

    let mut mbrpartition = MbrPartition::default();
    open_disk(DiskType::Master, 0, &mut mbrpartition as *mut MbrPartition);

    println!("MBR: {mbrpartition:?}\n");

    // This workaround is needed so the pointer `data` is correctly aligned
    let mut data_u16: [u16; 512 / core::mem::size_of::<u16>()] =
        [0; 512 / core::mem::size_of::<u16>()];
    let data = unsafe {
        core::slice::from_raw_parts_mut(data_u16.as_mut_ptr() as *mut u8, data_u16.len() * 2)
    };
    get_bytes(DiskType::Master, data, 1, 1);

    println!("First 10 bytes after MBR: {:?}", &data_u16[..10]);

    loop {}
}
