use core::arch::asm;

pub fn outportb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value
        );
    }
}

pub fn inportb(port: u16) -> u8 {
    let result: u8;
    unsafe {
        asm!(
            "in al, dx",
            out("al") result,
            in("dx") port
        );
    }
    result
}

pub fn inportw(port: u16) -> u16 {
    let result: u16;
    unsafe {
        asm!(
            "in ax, dx",
            inout("dx") port => _,
            out("ax") result
        );
    }
    result
}
