mod composition;
mod encapsulation;
mod polymorphism;
mod states;

use std::borrow::BorrowMut;

use composition::{Base, Child};
use states::Object;

use crate::polymorphism::Trait;

fn main() {
    println!("states");
    let mut obj = Object::new(String::from("this is string"), 234, 890);
    obj.print();
    obj.method1();
    obj.method2();
    obj.method3();
    obj.print();

    let mut b = Box::new(123);
    *b.as_mut() += 1;
    println!("{b}");

    println!("--------------");

    println!("encapsulation");
    let b = encapsulation::B::new();
    b.method_b();
    println!("{:?}", b.method_b_2());

    println!("--------------");
    polymorphism::static_polymorphism().print();
    polymorphism::dynamic_polymorphism().print();
    polymorphism::dynamic_polymorphism_boxed().print();
    println!("{:?}", polymorphism::dynamic_vec());

    println!("--------------");

    let c = composition::A::new("test");
    let s = composition::Struct::new();
    println!("{s:?}")
}
