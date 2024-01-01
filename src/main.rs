#![no_std]
#![no_main]
#![feature(panic_info_message)]

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
        ata::{ControllerType, DiskType},
        disk::Disk,
    },
    multiboot::MultibootInfo,
};

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("boot.S"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location() {
        println!("{}:{}: KERNEL PANIC!", loc.file(), loc.line());
    }
    // uses feature `panic_info_message`
    if let Some(msg) = info.message() {
        println!("  => {msg}");
    } else if let Some(payload) = info.payload().downcast_ref::<&'static str>() {
        println!("  => {payload}");
    }
    loop {}
}

#[no_mangle]
pub extern "C" fn init(multiboot_magic: u32, info: &MultibootInfo) -> ! {
    assert!(multiboot_magic == 0x2BADB002);
    unsafe { ALLOC.init(info) }

    let bootloader_name = info.get_bootloader_name();
    println!("[INI] Booted from bootloader `{bootloader_name}`");

    let disk = Disk::new(DiskType::Master, ControllerType::Master);

    let mbrpartition = disk.get_mbr_partition(0);
    println!("MBR: {mbrpartition:?}");

    if mbrpartition.typ == 0x0C {
        println!("Partition is of type FAT32 (with LBA addressing)");
    } else if mbrpartition.typ == 0x0B {
        println!("Partition is of type FAT32 (with CHS addressing)");
    }
    println!();

    // This workaround is needed so the pointer `data` is correctly aligned
    let mut data_u16: [u16; 512 / core::mem::size_of::<u16>()] =
        [0; 512 / core::mem::size_of::<u16>()];
    let data = unsafe {
        core::slice::from_raw_parts_mut(data_u16.as_mut_ptr() as *mut u8, data_u16.len() * 2)
    };
    disk.get_bytes(data, 1, 1);

    println!("First 10 bytes after MBR: {:?}", &data_u16[..10]);

    loop {}
}
