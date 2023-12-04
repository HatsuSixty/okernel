use crate::{drivers::ata::DiskType, print, println};

use super::ata::{read_sectors_pio, ControllerType};

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

pub struct Disk {
    pub typ: DiskType,
    pub controller: ControllerType,
}

impl Disk {
    pub fn new(disk_type: DiskType, controller_type: ControllerType) -> Self {
        Self {
            typ: disk_type,
            controller: controller_type,
        }
    }

    pub fn get_mbr_partition(&self, partition: usize, out: *mut MbrPartition) {
        println!(
            "[DIS] Getting MBR partition from disk {:?} on controller {:?}",
            self.typ, self.controller
        );
        let arr: &mut [u8] = &mut [0; SECTOR_SIZE];
        read_sectors_pio(self.typ, self.controller, arr, 0x0, 1);
        unsafe {
            let partition_info_offset = MBR_PARTITION_INDEXES[partition];
            *out = *(&arr[partition_info_offset] as *const u8 as *mut MbrPartition);
        }
    }

    pub fn get_bytes(&self, target_addr: &mut [u8], lba: u32, sector_count: u8) {
        read_sectors_pio(self.typ, self.controller, target_addr, lba, sector_count);
    }
}
