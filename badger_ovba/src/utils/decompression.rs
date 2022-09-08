use crate::error::Error;

// TODO Implement State/Context struct for for Compression algos

/// A utility function to handle decompression of compressed buffers according to
/// the [MS-OVBA] spec
pub fn decompress(compressed_buffer: &[u8]) -> Result<Vec<u8>, Error> {
    // compressed_buffer should be a CompressedContainer

    // consumable_buffer should now be an array of CompressedChunk (2.4.1.1.4)
    let mut decompressed_buffer = Vec::<u8>::with_capacity(4096);

    // decompress_compressed_container will check the signature byte, throwing an error
    // if it is not 0x01. It will return the array of Compressed Chunks
    let mut compressed_chunks = decompress_compressed_container(compressed_buffer)?;
    //println!("{:?}", compressed_chunks);

    while !compressed_chunks.is_empty() {
        compressed_chunks =
            decompress_compressed_chunk(compressed_chunks, &mut decompressed_buffer)?;
    }

    Ok(decompressed_buffer)
}

/// 2.4.1.1.4 CompressedContainer decompression function
///
/// A CompressedContainer consists of a Signature Byte follow by an array of Compressed Chunks
///
///  
fn decompress_compressed_container(compressed_container: &[u8]) -> Result<&[u8], Error> {
    // Check signature byte of compressed container
    let signature_byte = u8::from_le_bytes(compressed_container[..1].try_into().unwrap());
    if signature_byte != 0x01 {
        return Err(Error::Decompresssion(
            "Decompression Error: Invalid Signature Byte".into(),
        ));
    }
    // Consume the signature byte and return the array of CompressedChunks
    Ok(&compressed_container[1..])
}

/// 2.4.1.3.2 Decompressing a CompressedChunk
fn decompress_compressed_chunk<'a>(
    compressed_chunks: &'a [u8],
    decompressed_buffer: &mut Vec<u8>,
) -> Result<&'a [u8], Error> {
    let header = u16::from_le_bytes(compressed_chunks[..2].try_into().unwrap());

    let compressed_chunk_size = extract_compressed_chunk_size(header);
    let compressed_chunk_flag = extract_compressed_chunk_flag(header);
    let decompressed_chunk_start = decompressed_buffer.len();

    // The below is this implementations use of "Set CompressedCurrent to CompressedChunkStart + 2"
    let compressed_chunk_end = (compressed_chunk_size) as usize;
    if compressed_chunk_end > compressed_chunks.len() {
        return Err(Error::Decompresssion(
            "Compressed Chunk length exceeded the available amount of bytes".into(),
        ));
    }
    let mut current_compressed_chunk = &compressed_chunks[2..compressed_chunk_end];

    // Check to see if the compressed chunk flag is 1
    if compressed_chunk_flag == 1 {
        // Decompress a TokenSequence
        while !current_compressed_chunk.is_empty() {
            current_compressed_chunk = decompress_token_sequence(
                current_compressed_chunk,
                decompressed_buffer,
                decompressed_chunk_start,
            )?;
        }
    } else {
        // Decompress a RawChunk
        decompressed_buffer.extend_from_slice(current_compressed_chunk);
    }

    // We should be able to guruantee based off the compressed_chunk_end above that the below won't panic
    Ok(&compressed_chunks[compressed_chunk_end..])
}

fn decompress_token_sequence<'a>(
    current_compressed_chunk: &'a [u8],
    decompressed_buffer: &mut Vec<u8>,
    decompressed_chunk_start: usize,
) -> Result<&'a [u8], Error> {
    let flag_byte = u8::from_le_bytes(current_compressed_chunk[..1].try_into().unwrap());

    let mut current_token_chunk = &current_compressed_chunk[1..];

    if !current_token_chunk.is_empty() {
        for index in 0..8 as u8 {
            if current_token_chunk.is_empty() {
                return Ok(current_token_chunk);
            }
            // The below deviates from spec as it does not implement decompress_token in favor of
            // using some matches
            let flag = extract_flag_bit(index, flag_byte);

            match flag {
                0 => {
                    decompressed_buffer.push(current_token_chunk[0]);
                    current_token_chunk = &current_token_chunk[1..];
                }
                1 => {
                    let copy_token =
                        u16::from_le_bytes(current_token_chunk[..2].try_into().unwrap());
                    let (offset, length) = unpack_copy_token(
                        copy_token,
                        decompressed_chunk_start,
                        decompressed_buffer.len(),
                    );

                    // Deviating from specification below and not implementing ByteCopy
                    let mut copy_position = decompressed_buffer.len() - offset;
                    for _count in 0..length {
                        decompressed_buffer.push(decompressed_buffer[copy_position]);
                        copy_position += 1;
                    }
                    current_token_chunk = &current_token_chunk[2..];
                }
                _ => {
                    return Err(Error::Decompresssion(
                        "A flag token returned a value other than 0 or 1".into(),
                    ))
                }
            }
        }
    }

    Ok(current_token_chunk)
}

// Extractors -> Move to own module?

/// 2.4.1.3.12
fn extract_compressed_chunk_size(header: u16) -> u16 {
    let temp = header & 0x0FFF;
    temp + 3
}

/// 2.4.1.3.15
fn extract_compressed_chunk_flag(header: u16) -> usize {
    let temp = header & 0x8000;
    let compressed_flag = temp >> 15;
    compressed_flag as usize
}

/// 2.4.1.3.17
fn extract_flag_bit(index: u8, byte: u8) -> u8 {
    (byte >> index) & 1
}

/// 2.4.1.3.19.2
fn unpack_copy_token(
    copy_token: u16,
    decompressed_chunk_start: usize,
    decompressed_current: usize,
) -> (usize, usize) {
    let (length_mask, offset_mask, bit_count, _maximum_length) =
        copy_token_help(decompressed_chunk_start, decompressed_current);
    let length = (copy_token & length_mask) + 3;
    let offset = ((copy_token & offset_mask) >> (16 - bit_count)) + 1;

    (offset as usize, length as usize)
}

fn copy_token_help(
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
