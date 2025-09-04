pub struct BackReader<'a> {
    buffer: &'a [u8],
    cursor: usize,
}

impl<'a> BackReader<'a> {
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer,
            cursor: buffer.len(),
        }
    }

    pub fn read_slice(&mut self, b_to_read: usize) -> &'a [u8] {
        let end = self.cursor;
        let start = self.cursor.saturating_sub(b_to_read);

        &self.buffer[start..end]
    }

    pub fn move_cursor(&mut self, by: usize) {
        self.cursor = self.cursor.min(self.cursor + by);
    }
}
