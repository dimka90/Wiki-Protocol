pub fn encode(bytes: &[u8]) -> Option<Vec<u8>> {
    let len = bytes.len();
    let mut result = Vec::new();
    if len == 1 && bytes[0] <= 127 {
        result.extend_from_slice(bytes);
    } else if len <= 55 {
        result.push(0x80 + len as u8);
        result.extend_from_slice(bytes);
    }
    Some(result)
}
