use std::alloc::{alloc_zeroed, dealloc, Layout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Memory_Address(pub u64);

pub struct Memory {
    bytes: *mut u8,
    size:  usize,
}

impl Memory {
    pub fn new(size: usize) -> Self {
        let layout = Layout::from_size_align(size, 8).unwrap();
        let bytes = unsafe { alloc_zeroed(layout) };

        Self { bytes, size }
    }

    // #[inline(always)]
    // pub fn size(&self) -> usize {
    //     self.size
    // }

    #[inline(always)]
    pub fn read(&self, address: Memory_Address, data: &mut [u8]) {
        unsafe {
            let slot = self.bytes.add(address.0 as usize);

            slot.copy_to_nonoverlapping(data.as_mut_ptr(), data.len());
        }
    }

    #[inline(always)]
    pub fn write(&mut self, data: &[u8], address: Memory_Address) {
        unsafe {
            let slot = self.bytes.add(address.0 as usize);

            slot.copy_from_nonoverlapping(data.as_ptr(), data.len());
        }
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        let layout = Layout::from_size_align(self.size, 8).unwrap();

        unsafe { dealloc(self.bytes, layout) };
    }
}
