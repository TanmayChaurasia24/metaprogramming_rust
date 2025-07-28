use std::fmt::Error;
use serialize_macro::{SerializeNumberStruct, DeserializeNumberStruct};
use serialize_macro_traits::{Serialize, Deserialize};

#[derive(SerializeNumberStruct, DeserializeNumberStruct)]
struct Swap {
    qty_1: i32,
    qty_2: i32,
    qty_3: i32
}


fn main() {
    println!("Hello, world!");
    let s = Swap {
        qty_1: 1,
        qty_2: 2,
        qty_3: 1000
    };
    let bytes = s.serialize();
    println!("{:?}", bytes);
}