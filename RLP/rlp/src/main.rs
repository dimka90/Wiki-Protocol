pub  mod  rlp;

use crate::rlp::serilazation::{encode, decode_to_text};
fn main() {
    let data: &[u8] = "Expert will go extinct, if there are no Learners ".as_bytes();
    let result = encode(data);
    println!("Encoded data: {:?}", result);
    match decode_to_text(&result.unwrap()){
        Some(value) => println!("Decoded Data: {:?}", value),
        None => println!("NOne")
    };
}