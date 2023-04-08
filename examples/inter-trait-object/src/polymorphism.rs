use std::fmt::Debug;

pub trait Trait {
    fn print(&self);
}

#[derive(Debug)]
pub struct Struct;
impl Struct {
    pub fn new() -> Self {
        Struct
    }
}
impl Trait for Struct {
    fn print(&self) {
        println!("{:?}", self);
    }
}

// static polymorphism
pub fn static_polymorphism() -> impl Trait {
    Struct::new()
}

// dynamic polymorphism
pub fn dynamic_polymorphism<'a>() -> &'a dyn Trait {
    &Struct
}

/// more flexible trait object with boxing, so we don't have to mind the lifetime
pub fn dynamic_polymorphism_boxed() -> Box<dyn Trait> {
    Box::new(Struct)
}

pub trait Field: Debug {}

#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}

// vector that can store multiple different values
#[derive(Debug)]
pub struct DynamicVec {
    vec: Vec<Box<dyn Field>>,
}

pub fn dynamic_vec() -> DynamicVec {
    let mut v: Vec<Box<dyn Field>> = Vec::new();
    v.push(Box::new(123));
    v.push(Box::new("anu"));
    v.push(Box::new(String::from("nmnm")));
    v.push(Box::new(Custom));
    DynamicVec { vec: v }
}
