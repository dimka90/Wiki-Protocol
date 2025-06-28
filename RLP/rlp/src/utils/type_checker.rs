use std::any::{self, Any};
use std::fmt::Debug;
use crate::serialization::encode::{encode_string};
pub fn process_input<T: Any + Debug + >(input: &T){
      let any_input = input as &dyn Any;
    if any_input.is::<String> (){
        if let Some(input_value) = any_input.downcast_ref::<String>(){
                 println!("string values{:?}", encode_string(input_value));
        }
   
    }
    else if any_input.is::<i32>(){
        println!("This is an integer");
    }
    else{
        println!("Unknown data")
    }
}
// pub fn process_input<T: std::fmt::Debug>(input: &T)->  &'static str{
//     println!("Input to encode {:?}", input);
//  let input_type  = std::any::type_name::<T>();
//  input_type
// }