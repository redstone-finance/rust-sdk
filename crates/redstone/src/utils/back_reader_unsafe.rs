use core::mem::ManuallyDrop;

use crate::network::error::Error;

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
        // let vec = self.unsafe_read_vec(bytes_to_read);

        // decode_u64(&vec)

        unsafe { self.read_u64_direct(bytes_to_read) }
    }

    pub unsafe fn read_u64_direct(&mut self, bytes_to_read: usize) -> Result<u64, Error> {
        let start_offset = self.len.saturating_sub(bytes_to_read);
        let actual_bytes = self.len - start_offset;
        self.len = start_offset;

        let slice_ptr = self.pointer.add(start_offset);

        let start_idx = (0..actual_bytes)
            .find(|&i| *slice_ptr.add(i) != 0)
            .unwrap_or(actual_bytes);

        let significant_len = actual_bytes - start_idx;

        if significant_len > 8 {
            let slice = core::slice::from_raw_parts(slice_ptr.add(start_idx), significant_len);

            return Err(Error::NumberOverflow(slice.to_vec().into()));
        }

        let mut buffer = [0u8; 8];
        core::ptr::copy_nonoverlapping(
            slice_ptr.add(start_idx),
            buffer.as_mut_ptr().add(8 - significant_len),
            significant_len,
        );

        Ok(u64::from_be_bytes(buffer))
    }

    pub fn move_pointer(&mut self, by: usize) {
        self.len += by;
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
