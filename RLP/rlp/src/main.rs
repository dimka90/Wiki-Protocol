pub mod deserialization;
pub mod serialization;
pub mod utils;
use crate::deserialization::decode::{decode_to_text};
use crate::serialization::encode::encode_string;
use crate::utils::type_checker::process_input;
fn main() {
    let data = "Expert will go extinct, if there are no Learners ".to_string();
    let result = encode_string(&data);
    // process_input(&String::from("kkk"));
    println!("Encoded data: {:?}", result);
    match decode_to_text(&result.unwrap()){
        Some(value) => println!("Decoded Data: {:?}", value),
        None => println!("NOne")
    };
}