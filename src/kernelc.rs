use core::ffi::CStr;

#[no_mangle]
pub unsafe extern "C" fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let c = c as u8;
    for i in 0..n {
        *s.add(i) = c;
    }
    s
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    for i in 0..n {
        let a = *s1.add(i) as i32;
        let b = *s2.add(i) as i32;
        if a != b {
            return a - b;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8 {
    for i in 0..n {
        *d.add(i) = *s.add(i);
    }
    d
}

#[no_mangle]
pub unsafe extern "C" fn strlen(mut s: *const i8) -> usize {
    let mut result = 0;
    while *s != 0 {
        s = s.offset(1);
        result += 1;
    }
    result
}

#[no_mangle]
pub unsafe extern "C" fn rust_assert(expr: bool, file: *const i8, line: usize) {
    let cstr = unsafe { CStr::from_ptr(file) };
    let cstr = cstr.to_str().unwrap();
    if !expr {
        panic!("{cstr}:{line}: Assertion Failed");
    }
}
