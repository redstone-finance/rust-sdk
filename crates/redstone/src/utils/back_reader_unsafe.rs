use core::mem::ManuallyDrop;

use crate::{network::error::Error, utils::decode_utils::decode_u64};

pub struct UnsafeBackReader {
    pointer: *mut u8,
    len: usize,
}

impl UnsafeBackReader {
    pub fn new(payload: &mut [u8]) -> Self {
        let pointer = payload.as_mut_ptr();

        Self {
            pointer,
            len: payload.len(),
        }
    }

    pub fn unsafe_read_vec(&mut self, bytes_to_read: usize) -> ManuallyDrop<Vec<u8>> {
        unsafe {
            let start_offset = self.len.saturating_sub(bytes_to_read);
            let slice_ptr = self.pointer.add(start_offset);

            let bytes_to_read = self.len - start_offset;

            self.len = start_offset;

            ManuallyDrop::new(Vec::from_raw_parts(slice_ptr, bytes_to_read, bytes_to_read))
        }
    }

    pub fn unsafe_read_u64(&mut self, bytes_to_read: usize) -> Result<u64, Error> {
        let vec = self.unsafe_read_vec(bytes_to_read);

        decode_u64(&vec)
    }
}
