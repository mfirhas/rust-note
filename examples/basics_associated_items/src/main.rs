use std::fmt::Debug;
use std::fmt::Display;

fn main() {
    let resp = MyStruct::function1();
    println!("{}", resp);
    println!("{}", MyStruct::function3());
    let s = MyStruct {
        a: 123,
        b: String::from("lskmdf"),
    };
    println!("b: {}", s.b);
    println!("b: {}", s.function2());
    println!("{}", MyEnum::A("halo".to_string()).function2());
    println!("{}", MyEnum::B(123).function2());

    let _: MyStruct = MyTrait::method3();
    let _: MyStruct = <_ as MyTrait>::method3();
    let _ = <MyStruct as MyTrait>::method3();
    let _ = MyStruct::method3();

    // let ms = MyStruct {
    //     a: 123,
    //     b: String::from("lskdmf"),
    // };

    // ms.another_function1();

    println!("{}", MyStruct::method4().unwrap_err());

    println!("{}", MyStruct::D);
    println!("{}", <MyStruct as MyTrait>::D);

    let mut s = String::from("test");
    s.push_str("anu");
    println!("{}", s);

    println!("{}", MyStruct::C);
}

#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: String,
}

impl MyStruct {
    pub const C: &str = "nganu";
    pub fn function1() -> i32 {
        123
    }

    pub fn function2(mut self) -> String {
        self.b = String::from("Changed");
        self.b.clone()
    }

    fn function3() -> f32 {
        234_f32
    }
}

enum MyEnum {
    A(String),
    B(i32),
    C(f32),
}

impl MyEnum {
    pub fn function1() -> i32 {
        123
    }

    pub fn function2(&self) -> String {
        match *self {
            MyEnum::A(ref s) => s.clone(),
            _ => String::from("null"),
        }
    }
}

trait MyTrait {
    const D: i32 = 44;
    type Error: Display + Debug + Clone;

    fn method1() -> i32;
    fn method2(&self) -> String;
    fn method3() -> Self;
    fn method4() -> Result<(), Self::Error>;
}

impl MyTrait for MyStruct {
    const D: i32 = 123;
    type Error = String;

    fn method1() -> i32 {
        123
    }

    fn method2(&self) -> String {
        self.b.clone()
    }

    fn method3() -> Self {
        MyStruct {
            a: 123,
            b: "sdf".to_string(),
        }
    }

    fn method4() -> Result<(), Self::Error> {
        println!("const: {}", Self::D);
        return Err("error bung".to_string());
    }
}

// impl MyStruct {
//     pub const A: &str = "anu";

//     fn another_function1(self: <MyStruct as MyTrait>::Output) {
//         println!("ANU:: {:?}", self);
//     }

// }

// mod a {

//     use super::{MyStruct, MyTrait};

//     fn sdf() {
//         let d = MyStruct::method3();
//         let ms = MyStruct {
//             a: 123,
//             b: String::from("lskdmf"),
//         };

//         ms.another_function1();
//     }
// }
