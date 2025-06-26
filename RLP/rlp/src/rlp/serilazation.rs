use rlp::{Rlp};
pub fn encode(bytes:&[u8]) -> Option<Vec<u8>>{
let len = bytes.len();
let mut result = Vec::new();
if len == 1  && bytes[0] <=127 {
    result.extend_from_slice(bytes);
}
else if len <= 55 {
    result.push(0x80 + len as u8);
    result.extend_from_slice(bytes);
}
Some(result)
}

pub fn decode_to_hex(encoded_data:&[u8]) -> Option<String>{
    let encoded_data = encoded_data.iter()
    .map(|single_byte | format!("{:02x}", single_byte))
    .collect::<String>();
    Some(encoded_data)
}


pub fn decode_to_text(encoded_data:&[u8]) -> Option<String>{
    let rlp = Rlp::new(&encoded_data);
    let decoded: &[u8] = rlp.data().unwrap();
    let result = String::from_utf8(decoded.to_vec()).unwrap();
    Some(result)
}