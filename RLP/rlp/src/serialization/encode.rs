// pub fn encode(bytes: &[u8]) -> Option<Vec<u8>> {
//     let len = bytes.len();
//     let mut result = Vec::new();
//     if check_len(bytes, bytes[0]){
//         result.extend_from_slice(bytes);
//     } else if len <= 55 {
//         result.push(0x80 + len as u8);
//         result.extend_from_slice(bytes);
//     }
//     Some(result)
// } 
pub fn encode_string(input_data: &String) -> Option<Vec<u8>>{
    let mut result  = Vec::new();
        let string_bytes = input_data.as_bytes();
    if input_data.is_empty() || input_data.len() == 0{
        println!(" i am empty");
        result.push(0x80 as u8);
    }
    else if check_len(string_bytes, string_bytes[0]){
        result.extend_from_slice(string_bytes);
    }
else if string_bytes.len() <=55 {
    result.push(0x80 + string_bytes.len() as u8);
    result.extend_from_slice(string_bytes);    
}

    Some(result)
}

pub fn check_len(bytes: &[u8], byte_value: u8) -> bool{
    let len = bytes.len();
    if len == 1 && byte_value <=127 {
        true
    }
    else {
    false
    }
}