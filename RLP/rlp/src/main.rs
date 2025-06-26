pub  mod  rlp;

use crate::rlp::serilazation::{encode, decode_to_hex, decode_to_text};
fn main() {
    let data: &[u8] = "h".as_bytes();
    let result = encode(data);
    println!("Bytes array{:?}", result );
    match decode_to_text(&result.unwrap()){
        Some(value) => println!("{:?}", value),
        None => println!("NOne")
    };
}