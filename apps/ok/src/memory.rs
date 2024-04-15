use std::alloc::{alloc_zeroed, dealloc, Layout};

pub struct Memory {
    bytes: *mut u8,
    size: usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let bytes = unsafe { alloc_zeroed(layout) };

        Self { bytes, size }
    }

    pub fn slot(&self, address: Memory_Address) -> *const u8 {
        unsafe { self.bytes.add(address.0) }
    }

    pub fn slot_mut(&mut self, address: Memory_Address) -> *mut u8 {
        unsafe { self.bytes.add(address.0) }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, 8).unwrap();

        unsafe { dealloc(self.bytes, layout) };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Memory_Address(pub usize);
