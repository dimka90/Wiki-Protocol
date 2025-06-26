pub  mod  rlp;

use crate::rlp::serilazation::{encode, decode_to_text};
fn main() {
    let data: &[u8] = "Dimka is  the next expect".as_bytes();
    let result = encode(data);
    match decode_to_text(&result.unwrap()){
        Some(value) => println!("{:?}", value),
        None => println!("NOne")
    };
}