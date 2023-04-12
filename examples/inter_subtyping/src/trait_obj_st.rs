use std::fmt::Debug;

pub fn trait_object() {
    let mut v: Vec<Box<dyn Field>> = Vec::new();
    v.push(Box::new(123));
    v.push(Box::new("anu"));
    v.push(Box::new(String::from("nmnm")));
    v.push(Box::new(Custom));

    println!("{:?}", v);
}

pub trait Field: Debug {}
#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}
