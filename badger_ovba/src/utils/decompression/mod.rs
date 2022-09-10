use crate::error::Error;

use crate::utils::copy_token_help;
// TODO Implement State/Context struct for for Compression algos

/// A utility function to handle decompression of compressed buffers according to
/// the [MS-OVBA] spec
pub fn decompress(compressed_buffer: &[u8]) -> Result<Vec<u8>, Error> {
    // compressed_buffer should be a CompressedContainer

    // consumable_buffer should now be an array of CompressedChunk (2.4.1.1.4)
    let mut decompressed_buffer = Vec::<u8>::new();
    // decompress_compressed_container will check the signature byte, throwing an error
    // if it is not 0x01. It will return the array of Compressed Chunks
    let mut compressed_chunks = decompress_compressed_container(compressed_buffer)?;

    let mut decompressed_current = decompressed_buffer.len();
    while !compressed_chunks.is_empty() {
        let mut decompressed_chunk = Vec::<u8>::with_capacity(4096);

        compressed_chunks = decompress_compressed_chunk(
            compressed_chunks,
            &mut decompressed_chunk,
            decompressed_current,
        )?;

        decompressed_buffer.append(&mut decompressed_chunk);
        // Update DecompressedCurrent after appending DecompressedChunk onto the DecompressedBuffer
        decompressed_current = decompressed_buffer.len();
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
    decompressed_chunk: &mut Vec<u8>,
    decompressed_current: usize,
) -> Result<&'a [u8], Error> {
    let header = u16::from_le_bytes(compressed_chunks[..2].try_into().unwrap());

    let compressed_chunk_size = extract_compressed_chunk_size(header);
    let compressed_chunk_flag = extract_compressed_chunk_flag(header);
    let decompressed_chunk_start = decompressed_current;

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
                decompressed_chunk,
                decompressed_chunk_start,
            )?;
        }
    } else {
        // Decompress a RawChunk
        decompressed_chunk.extend_from_slice(current_compressed_chunk);
    }

    // We should be able to guruantee based off the compressed_chunk_end above that the below won't panic
    Ok(&compressed_chunks[compressed_chunk_end..])
}

fn decompress_token_sequence<'a>(
    current_compressed_chunk: &'a [u8],
    decompressed_chunk: &mut Vec<u8>,
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
                    decompressed_chunk.push(current_token_chunk[0]);
                    current_token_chunk = &current_token_chunk[1..];
                }
                1 => {
                    let copy_token =
                        u16::from_le_bytes(current_token_chunk[..2].try_into().unwrap());
                    let (offset, length) = unpack_copy_token(
                        copy_token,
                        decompressed_chunk_start,
                        decompressed_chunk.len(),
                    );

                    // Deviating from specification below and not implementing ByteCopy
                    let mut copy_position = decompressed_chunk.len() - offset;
                    for _count in 0..length {
                        decompressed_chunk.push(decompressed_chunk[copy_position]);
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
