use crate::error::Error;

pub(crate) mod state;
use crate::utils::copy_token_help;
use state::CompressionState;
// Compression may have to be appraoched differently and more according to Spec.
// We cannot slowly consume the decompressed array
pub fn compress(decompressed_buffer: &[u8]) -> Result<Vec<u8>, Error> {
    let signature_byte: u8 = 0x01;
    let mut compressed_buffer = Vec::<u8>::new();
    // we set the signature byte that signifies the beginning of the CompressedContainer
    compressed_buffer.push(signature_byte);
    let mut state = CompressionState::new(&compressed_buffer);

    // Conerns about going directly based off specification -> We are not declaring the size of our vector, but then indexing into it at a further point.
    // The better approach would probably be to call a compressed_chunks_buffer with a capacity and operate using that capacity.
    //
    // This capacity appraoch may be the better approach. Below will be commented out code/pseudocode for the design

    while state.decompressed_current < decompressed_buffer.len() {
        state.decompressed_chunk_start = state.decompressed_current;
        state.compressed_chunk_start = state.compressed_current; // <- remove this line of code in case of Capacity approach

        compress_decompressed_chunk(&decompressed_buffer, &mut compressed_buffer, &mut state);
        // returns mut compressed_chunk: Vec<u8>
        // compressed_buffer.append(&mut compressed_chunk)
    }

    Ok(compressed_buffer)
}

// NOTE: Need ot make sure that the compression takes account of the current index and resetting of chunk
fn compress_decompressed_chunk<'a>(
    decompressed_buffer: &'a [u8],
    compressed_buffer: &mut Vec<u8>,
    state: &mut CompressionState,
) {
    // let mut Vec::<u8>::with_capacity(4098);
    state.compressed_end = state.compressed_current + 4098; // CompressedEnd would just be 4098, would be something that could be refactored/removed down the line
    state.compressed_current = state.compressed_chunk_start + 2; // CompressedChunk start would always be 0, so variable unneeded. CompressedCurrent = 2
    state.decompressed_end = if state.decompressed_chunk_start + 4096 < decompressed_buffer.len() {
        state.decompressed_chunk_start + 4096
    } else {
        decompressed_buffer.len()
    };

    let mut compressed_flag: u16 = 1;

    while state.decompressed_current < state.decompressed_end
        && state.compressed_current < state.compressed_end
    {
        // Call Compressing a TokenSequence
        compress_decompressed_token_sequence(decompressed_buffer, compressed_buffer, state);
    }

    if state.decompressed_current < state.decompressed_end {
        // Call Compressing RawChunk
        compress_raw_chunk(decompressed_buffer, compressed_buffer, state);
        compressed_flag = 0
    }

    let size = state.compressed_current as u16 - state.compressed_chunk_start as u16;
    let mut header: u16 = 0x0000;
    header = pack_compressed_chunk_size(size, header);
    header = pack_compressed_chunk_flag(compressed_flag, header);
    header = pack_compressed_chunk_sig(header);

    compressed_buffer[state.compressed_chunk_start] = header as u8;
    compressed_buffer[state.compressed_chunk_start + 1] = (header >> 8) as u8;
}

fn compress_decompressed_token_sequence(
    decompressed_buffer: &[u8],
    compressed_buffer: &mut Vec<u8>,
    state: &mut CompressionState,
) {
    let flag_byte_i = state.compressed_current;
    let mut token_flags = 0b0 as u8;
    state.compressed_current += 1;

    for _index in 0..8 as usize {
        if state.decompressed_current < decompressed_buffer.len()
            && state.compressed_current < state.compressed_end
        {
            token_flags = compress_token(token_flags, decompressed_buffer, compressed_buffer, state)
        }
    }
    compressed_buffer[flag_byte_i] = token_flags;
}

fn compress_token(
    flags: u8,
    decompressed_buffer: &[u8],
    compressed_buffer: &mut Vec<u8>,
    state: &mut CompressionState,
) -> u8 {
    let mut offset = 0 as usize;
    let (offset, length) = matching(decompressed_buffer, state);
    if offset != 0 {
        if state.compressed_current + 1 < state.compressed_end {
            // deviating by not implementing pack_copy_token
            let (_l_mask, _o_mask, bit_count, _max_length) =
                copy_token_help(state.decompressed_chunk_start, state.decompressed_current);
            let one = offset as u16 - 1;
            let two = 16 - bit_count;
            let three = length as u16 - 3;
            let token = (one << two) | three;
            // Add the token bytes to the array in little endian
            compressed_buffer[state.compressed_current] = token as u8;
            compressed_buffer[state.compressed_current + 1] = (token >> 8) as u8;
            state.compressed_current += 2;
            state.decompressed_current += length;

            let set_flag = set_flag_bit(1, flags);
            return set_flag;
        } else {
            state.compressed_current = state.compressed_end
        }
    } else {
        if state.compressed_current < state.compressed_end {
            compressed_buffer[state.compressed_current] =
                decompressed_buffer[state.decompressed_current];
            state.compressed_current += 1;
            state.decompressed_current += 1;
        }
    }
    flags
}

fn compress_raw_chunk(
    decompressed_buffer: &[u8],
    compressed_buffer: &mut Vec<u8>,
    state: &mut CompressionState,
) {
    state.compressed_current = state.compressed_chunk_start + 2;
    state.decompressed_current = state.decompressed_chunk_start;
    let mut pad_count = 4096 as usize;

    for byte_index in (state.decompressed_chunk_start..state.decompressed_end) {
        compressed_buffer[state.compressed_current] = decompressed_buffer[byte_index];
        state.compressed_current += 1;
        state.decompressed_current += 1;
        pad_count -= 1;
    }

    // Pad out the rest of the remaining chunk with 0x00 (zeroes)
    for _count in 0..pad_count {
        compressed_buffer[state.compressed_current] = 0x00;
        state.compressed_current += 1;
    }
}

fn matching(decompressed_buffer: &[u8], state: &mut CompressionState) -> (usize, usize) {
    let mut candidate = state.decompressed_current - 1;
    let mut best_length = 0 as usize;
    let mut best_candidate = 0 as usize;

    while state.decompressed_current <= candidate {
        let mut c = candidate;
        let mut d = state.decompressed_current;
        let mut len = 0 as usize;
        while d < decompressed_buffer.len() && decompressed_buffer[d] == decompressed_buffer[c] {
            c += 1;
            d += 1;
            len += 1;
        }

        if len > best_length {
            best_length = len;
        }
        candidate -= 1;
    }

    if best_length >= 3 {
        //call copy_token_help
        let (length_mask, offset_mask, bit_count, max_length) =
            copy_token_help(state.decompressed_chunk_start, state.decompressed_current);
        let length = if best_length < max_length as usize {
            best_length
        } else {
            max_length as usize
        };
        let offset = state.decompressed_current - best_candidate;
        (length, offset)
    } else {
        (0, 0)
    }
}

fn pack_compressed_chunk_size(size: u16, header: u16) -> u16 {
    let temp = header & 0xF000;
    let temp_2 = size - 3;
    temp | temp_2
}

fn pack_compressed_chunk_sig(header: u16) -> u16 {
    let temp = header | 0x8FFF;
    temp | 0x3000
}

fn pack_compressed_chunk_flag(compressed_flag: u16, header: u16) -> u16 {
    let temp = header & 0x7FFF;
    let two = compressed_flag << 15;
    (temp | two)
}

fn set_flag_bit(index: u8, byte: u8) -> u8 {
    let temp = byte << index;
    let temp_2 = byte & (!temp);
    (temp | temp_2)
}
