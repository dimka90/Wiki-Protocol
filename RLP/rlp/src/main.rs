pub mod deserialization;
pub mod serialization;
use crate::deserialization::decode::{decode_to_text};
use crate::serialization::encode::encode;
fn main() {
    let data: &[u8] = "Expert will go extinct, if there are no Learners ".as_bytes();
    let result = encode(data);
    println!("Encoded data: {:?}", result);
    match decode_to_text(&result.unwrap()){
        Some(value) => println!("Decoded Data: {:?}", value),
        None => println!("NOne")
    };
}