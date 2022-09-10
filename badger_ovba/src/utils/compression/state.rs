pub(crate) struct CompressionState {
    pub compressed_current: usize,
    pub decompressed_current: usize,
    pub compressed_chunk_start: usize,
    pub decompressed_chunk_start: usize,
    pub compressed_end: usize,
    pub decompressed_end: usize,
}

impl CompressionState {
    pub(crate) fn new(compressed: &[u8]) -> Self {
        let compressed_current = compressed.len();

        Self {
            compressed_current,
            decompressed_current: 0 as usize,
            compressed_chunk_start: compressed_current,
            decompressed_chunk_start: 0 as usize,
            compressed_end: 0,
            decompressed_end: 0,
        }
    }

    // Can probably be deleted, but keeping for now
    pub(crate) fn set(&mut self, id: &str, new_value: usize) {
        match id {
            "compressed_current" => self.compressed_current = new_value,
            "decompressed_current" => self.decompressed_current = new_value,
            "compressed_chunk_start" => self.compressed_chunk_start = new_value,
            "decompressed_chunk_start" => self.decompressed_chunk_start = new_value,
            _ => {}
        }
    }
}
