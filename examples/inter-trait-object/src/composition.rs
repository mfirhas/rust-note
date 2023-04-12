use std::fmt::Debug;

/// composition
///
///
pub trait IDep: Debug {}
pub trait Field: Debug {}
#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}

#[derive(Debug)]
pub struct Struct {
    field1: Box<dyn IDep>,
    field2: Vec<Box<dyn Field>>,
}

#[derive(Debug)]
pub struct Dep;
impl IDep for Dep {}

impl Struct {
    pub fn new() -> Self {
        let mut v: Vec<Box<dyn Field>> = Vec::new();
        v.push(Box::new(123));
        v.push(Box::new("anu"));
        v.push(Box::new(String::from("nmnm")));
        v.push(Box::new(Custom));
        Self {
            field1: Box::new(Dep),
            field2: v,
        }
    }
}

