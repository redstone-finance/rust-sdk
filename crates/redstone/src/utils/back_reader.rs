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
        self.cursor = self.cursor.saturating_sub(b_to_read);

        &self.buffer[self.cursor..end]
    }

    pub fn set_cursor(&mut self, to: usize) {
        self.cursor = to;
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }
}
