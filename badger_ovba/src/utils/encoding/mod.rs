use codepage;
use encoding_rs::*;

pub fn convert_mbcs_value(bytes_to_convert: &[u8]) -> Vec<u8> {
    let encoding = codepage::to_encoding(1252 as u16).unwrap();
    let mut output = [0 as u8; 2048];
    let mut decoder = encoding.new_decoder_with_bom_removal();

    // Todo: build out error handling regarding the below outputs
    let (_result, _read, _written, _had_errors) =
        decoder.decode_to_utf8(&bytes_to_convert[..], &mut output, false);
    

    output[..bytes_to_convert.len()].to_vec()
}
