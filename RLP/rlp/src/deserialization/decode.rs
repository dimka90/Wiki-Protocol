use rlp::Rlp;
pub fn decode_to_hex(encoded_data: &[u8]) -> Option<String> {
    let encoded_data = encoded_data
        .iter()
        .map(|single_byte| format!("{:02x}", single_byte))
        .collect::<String>();
    Some(encoded_data)
}

pub fn decode_to_text(encoded_data: &[u8]) -> Option<String> {
    let rlp = Rlp::new(&encoded_data);
    let decoded: &[u8] = rlp.data().unwrap();
    let result = String::from_utf8(decoded.to_vec()).unwrap();
    Some(result)
}
