use crate::{drivers::ata::DiskType, print, println};

use super::ata::read_sectors_pio;

const SECTOR_SIZE: usize = 512;

const MBR_PARTITION_1: usize = 0x01BE;
const MBR_PARTITION_2: usize = 0x01CE;
const MBR_PARTITION_3: usize = 0x01DE;
const MBR_PARTITION_4: usize = 0x01EE;

const MBR_PARTITION_INDEXES: [usize; 4] = [
    MBR_PARTITION_1,
    MBR_PARTITION_2,
    MBR_PARTITION_3,
    MBR_PARTITION_4,
];

#[derive(Clone, Copy, Debug, Default)]
#[repr(C, packed)]
pub struct MbrPartition {
    pub status: u8,
    pub chs_first_sector: [u8; 3],
    pub typ: u8,
    pub chs_last_sector: [u8; 3],
    pub lba_first_sector: u32,
    pub sector_count: u32,
}

pub fn get_bytes(disk_type: DiskType, target_addr: &mut [u8], lba: u32, sector_count: u8) {
    read_sectors_pio(disk_type, target_addr, lba, sector_count)
}

pub fn open_disk(disk_type: DiskType, partition: usize, out: *mut MbrPartition) {
    println!("[DIS] Reading sectors from MASTER disk");
    let arr: &mut [u8] = &mut [0; SECTOR_SIZE];
    read_sectors_pio(disk_type, arr, 0x0, 1);
    unsafe {
        let partition_info_offset = MBR_PARTITION_INDEXES[partition];
        *out = *(&arr[partition_info_offset] as *const u8 as *mut MbrPartition);
    }
}
