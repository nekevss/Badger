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
}
