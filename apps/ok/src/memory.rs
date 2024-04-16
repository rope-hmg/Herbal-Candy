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

    #[inline(always)]
    pub fn write(&mut self, data: &[u8], address: Memory_Address) {
        let slot = self.slot_mut(address);

        unsafe {
            slot.copy_from(data.as_ptr(), data.len());
        }
    }

    #[inline(always)]
    pub fn slot(&self, address: Memory_Address) -> *const u8 {
        unsafe { self.bytes.add(address.0) }
    }

    #[inline(always)]
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
