use std::fmt::{Debug, Display};

fn main() {
    print::<i32>(123);

    let r = ret::<i32>(1);
    println!("{:?}",r);
    let r = ret::<f32>(1_f32);
    println!("{:?}",r);
    let r = ret(123);
    println!("{:?}",r);
    let r: f32 = ret(123_f32);
    println!("{:?}",r);

    let a = A {
        a: 123,
        b: 34,
    };

    let a = A::<f32> {
        a: 123.2,
        b: 23.0,
    };

    let a2 = A2::<i32,f32> {
        a: 123,
        b: 34.4,
    };

    let a2 = A2 {
        a: 123,
        b: 34.5,
    };

    let b = B::<i32>::Field2(123);
    println!("{:?}",b);

    let b2 = B2::<i32, f32>::Field2(123);
    let b2: B2<&str, i32> = B2::Field2("123");

    // let d: D<i32> = 123;
    // d::method1(123);
    let a = A::<i32> {
        a: 123,
        b: 234,
    };
    a.method2(123);
    let a = A::<i32>::method1(123);

    println!("-------------");

    let ms = MyStruct {
        a: 1,
        b: String::from("test"),
    };
    dbg!(ms.method1());
}

pub trait MyTrait {
    fn method1(&self) -> i32;
    fn method2(param1: &str) -> Result<(),String>;
    fn method3(param1: String);
}

pub struct MyStruct {
    a: i32,
    b: String,
}

impl MyTrait for MyStruct {
    fn method1(&self) -> i32 {
        self.a
    }

    fn method2(param1: &str) -> Result<(),String> {
        if param1.is_empty() {
            return Err("empty string".to_string());
        }
        Ok(())
    }

    fn method3(param1: String) {
        println!("do nothing");
    }
}

// ------------

trait D<T> {
    fn method1(param: T);
}

impl D<i32> for i32 {
    fn method1(param: i32) {
        print!("{param}");
    }
}

#[derive(Debug)]
enum B<T> {
    Field1,
    Field2(T),
    Field3 {
        t: T,
    }
}

#[derive(Debug)]
enum B2<T, E> {
    Field1,
    Field2(T),
    Field3 {
        t: E,
    }
}

fn print<T: Debug + Display>(param: T) {
    println!("{:?}",param);
}

struct A<T> {
    a: T,
    b: T,
}

impl<T> A<T> {
    fn method1(param: T) -> T {
        param
    }

    fn method2(&self, param: T) {
        // ...
    }
}

struct A2<T, E> {
    a: T,
    b: E,
}

trait Number {}

impl Number for i32 {}

impl Number for f32 {}

fn ret<T: Number>(number: T) -> T {
    number
}