use crate::o1heap::*;
use core::{
    alloc::{GlobalAlloc, Layout},
    ffi::c_void,
};

#[allow(improper_ctypes)]
extern "C" {
    static KERNEL_START: ();
    static KERNEL_END: ();
}

use crate::multiboot::MultibootInfo;

#[global_allocator]
pub static mut ALLOC: Allocator = Allocator::new();

pub struct Allocator {
    o1heap: *mut O1HeapInstance,
}

impl Allocator {
    pub const fn new() -> Self {
        Self {
            o1heap: core::ptr::null_mut(),
        }
    }

    pub fn init(&mut self, info: &MultibootInfo) {
        let big_chunk = if let Some(chunk) = info
            .get_mmap_entries()
            .iter()
            .find(|x| unsafe { x.addr as *const () == &KERNEL_START as *const () })
        {
            chunk
        } else {
            panic!("Could not find big block of RAM");
        };

        let mut kernel_end_addr = unsafe { (&KERNEL_END as *const ()) as u32 };
        let kernel_start_addr = unsafe { (&KERNEL_START as *const ()) as u32 };
        let reserved_memory_length = (kernel_end_addr - kernel_start_addr) as usize;
        let segment_size = big_chunk.len as usize - reserved_memory_length;

        let aligment = core::mem::size_of::<*mut u8>() * 4 as usize;
        kernel_end_addr += aligment as u32 - (kernel_end_addr % aligment as u32);

        let o1heap_instance =
            unsafe { o1heapInit(kernel_end_addr as *mut c_void, segment_size) };
        if o1heap_instance.is_null() {
            panic!("Could not initialize memory allocator");
        }
        self.o1heap = o1heap_instance;
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        return o1heapAllocate(self.o1heap, layout.size()) as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _: Layout) {
        o1heapFree(self.o1heap, ptr as *mut c_void);
    }
}
