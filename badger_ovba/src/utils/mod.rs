pub mod compression;
pub mod decompression;
pub mod encoding;

pub use compression::compress;
pub use decompression::decompress;
pub use encoding::convert_mbcs_value;

pub(crate) fn copy_token_help(
    decompressed_chunk_start: usize,
    decompressed_current: usize,
) -> (u16, u16, u16, u16) {
    let diff = decompressed_current - decompressed_chunk_start;
    // Calculate the value that is >= log base 2 of difference
    let mut bit_count = 4 as u16;
    while 1 << bit_count < diff {
        bit_count += 1;
    }
    // Note: there's a weird line in the pseudocode that is not entirely clear
    // Set BitCount TO the maximum of BitCount and 4 ... this may have something
    // to do with bit_count needing to be set to a minimum of 4

    let length_mask = 0xFFFF as u16 >> bit_count;
    let offset_mask = !length_mask;
    let maximum_length = (0xFFFF as u16 >> bit_count) + 3;

    (length_mask, offset_mask, bit_count, maximum_length)
}
