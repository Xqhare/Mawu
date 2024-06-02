use std::collections::HashMap;


pub enum MawuValue {
    Object(HashMap<String, MawuValue>),
    Array(Vec<MawuValue>),
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
    Null,
}
    
