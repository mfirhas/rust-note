use std::fmt::Debug;

pub trait MyTrait {
    fn method(&self) -> String;
}

pub trait MyTrait2 {
    fn method(&self) -> String;
}

impl<T: Debug> MyTrait for &mut T {
    fn method(&self) -> String {
        println!("---");
        println!("from MyTrait method for &mut T {:?}", self);
        String::from("this")
    }
}

impl<T: Debug> MyTrait2 for T {
    fn method(&self) -> String {
        println!("---");
        println!("from MyTrait method for T {:?}", self);
        String::from("this")
    }
}