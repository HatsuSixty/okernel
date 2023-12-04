use crate::cpu::{inportb, inportw, outportb};

const STATUS_BSY: u8 = 0x80;
const STATUS_RDY: u8 = 0x40;

const ATA_MASTER_BASE: u16 = 0x1F0;
//const ATA_SLAVE_BASE: u16 = 0x170;

const ATA_MASTER: u8 = 0xE0;
const ATA_SLAVE: u8 = 0xF0;

//const ATA_REG_DATA: u8 = 0x00;
//const ATA_REG_ERROR: u8 = 0x01;
//const ATA_REG_FEATURES: u8 = 0x01;
const ATA_REG_SECCOUNT0: u8 = 0x02;
const ATA_REG_LBA0: u8 = 0x03;
const ATA_REG_LBA1: u8 = 0x04;
const ATA_REG_LBA2: u8 = 0x05;
const ATA_REG_HDDEVSEL: u8 = 0x06;
const ATA_REG_COMMAND: u8 = 0x07;
//const ATA_REG_STATUS: u8 = 0x07;
//const ATA_REG_SECCOUNT1: u8 = 0x08;
//const ATA_REG_LBA3: u8 = 0x09;
//const ATA_REG_LBA4: u8 = 0x0A;
//const ATA_REG_LBA5: u8 = 0x0B;
//const ATA_REG_CONTROL: u8 = 0x0C;
//const ATA_REG_ALTSTATUS: u8 = 0x0C;
//const ATA_REG_DEVADDRESS: u8 = 0x0D;

fn wait_bsy() {
    // wait for BSY to be 0
    while (inportb(0x1F7) & STATUS_BSY) != 0 {}
}

const MAX_WAIT_DRQ_ATTEMPTS: usize = 1000000;
fn wait_drq() {
    // wait for DRQ to be 1
    let mut attempts = 0;
    while (inportb(0x1F7) & STATUS_RDY) == 0 {
        if attempts >= MAX_WAIT_DRQ_ATTEMPTS {
            panic!("[ATA] wait_drq(): timed out");
        }
        attempts += 1;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum DiskType {
    Master,
    Slave,
}

pub fn read_sectors_pio(disk_type: DiskType, target_addr: &mut [u8], lba: u32, sector_count: u8) {
    wait_bsy();
    // 0xE0 -> master, 0xF0 -> slave, 4 highest bits of LBA
    let disk = match disk_type {
        DiskType::Master => ATA_MASTER,
        DiskType::Slave => ATA_SLAVE,
    };
    outportb(
        ATA_MASTER_BASE + ATA_REG_HDDEVSEL as u16,
        (disk as u32 | ((lba >> 24) & 0xF)) as u8,
    );
    // Send the amount of sectors we want
    outportb(ATA_MASTER_BASE + ATA_REG_SECCOUNT0 as u16, sector_count);
    // Send LBA, 8 bits at a time
    outportb(ATA_MASTER_BASE + ATA_REG_LBA0 as u16, lba as u8);
    outportb(ATA_MASTER_BASE + ATA_REG_LBA1 as u16, (lba >> 8) as u8);
    outportb(ATA_MASTER_BASE + ATA_REG_LBA2 as u16, (lba >> 16) as u8);
    // Read
    outportb(ATA_MASTER_BASE + ATA_REG_COMMAND as u16, 0x20);

    let mut target = target_addr.as_mut_ptr() as *mut u16;

    for _ in 0..sector_count {
        wait_bsy();
        wait_drq();
        for i in 0..256 {
            unsafe {
                *target.add(i) = inportw(0x1F0);
            }
        }
        target = unsafe { target.add(256) };
    }
}
