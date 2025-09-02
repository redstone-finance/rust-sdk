use alloc::vec::Vec;

use crate::{network::error::Error, FeedId};

pub struct ReverseReader {
    data: Vec<u8>,
    cursor: usize,
}

impl ReverseReader {
    pub fn new(data: Vec<u8>) -> Self {
        let cursor = data.len();

        Self { data, cursor }
    }
    pub fn remaining_len(&self) -> usize {
        self.cursor
    }

    pub fn read_slice(&mut self, size: usize) -> Result<&[u8], Error> {
        if size > self.cursor {
            return Err(Error::UnexpectedBufferEnd);
        }

        self.cursor -= size;

        Ok(&self.data[self.cursor..self.cursor + size])
    }

    pub fn read_vec(&mut self, len: usize) -> Result<Vec<u8>, Error> {
        if len > self.cursor {
            return Err(Error::UnexpectedBufferEnd);
        }

        if len == 0 {
            return Ok(Vec::new());
        }

        self.cursor -= len;
        let slice = &self.data[self.cursor..self.cursor + len];

        Ok(slice.to_vec())
    }

    pub fn read_u64(&mut self, len: usize) -> Result<u64, Error> {
        if len > self.cursor {
            return Err(Error::UnexpectedBufferEnd);
        }

        self.cursor -= len;
        let y = &self.data[self.cursor..self.cursor + len];

        if y.len() > 8 {
            return Err(Error::NumberOverflow(y.to_vec().into()));
        }
        let mut buff = [0; 8];
        buff[8 - y.len()..].copy_from_slice(y);

        Ok(u64::from_be_bytes(buff))
    }

    pub fn read_feed_id(&mut self, len: usize) -> Result<FeedId, Error> {
        if len > self.cursor {
            return Err(Error::UnexpectedBufferEnd);
        }

        self.cursor -= len;
        let slice = &self.data[self.cursor..self.cursor + len];

        Ok(FeedId::from(slice.to_vec()))
    }

    pub fn set_cursor(&mut self, to: usize) -> Result<(), Error> {
        if to > self.data.len() {
            return Err(Error::UnexpectedBufferEnd);
        }

        self.cursor = to;

        Ok(())
    }

    #[cfg(test)]
    pub fn remaining_data(self) -> Vec<u8> {
        self.data[0..self.cursor].to_vec()
    }
}
