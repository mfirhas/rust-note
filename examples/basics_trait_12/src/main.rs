mod a;
mod ass;
use a::{Enum, Struct, Trait};
mod default;
use ass::TraitAss;
use default::{D, O, P};

fn main() {
    let ms = MyStruct {
        a: 1,
        b: String::from("test"),
    };
    dbg!(ms.method1());

    let ret = MyStruct::method2("param1");
    dbg!(ret);

    MyStruct::method3("nothing".to_string());

    // -----------

    accept_trait(&ms);

    let t = return_trait();
    dbg!(t.method1());

    dbg!(F::return_trait_from_impl().method1());

    // -----------

    let ret = <MyStruct as MyTrait>::method2("param1");

    // -----------
    let sss = Struct {
        s: "sdf".to_string(),
    };
    sss.t();
    Struct::t2();

    // -----------

    let e = Enum::H("anu".to_string());
    e.t();
    Enum::t2();

    // -----------

    let num = 123;
    println!("{}", num.t());
    i32::t2();

    // -----------
    let slice: &[i32] = &[1, 2, 3, 4];
    println!("{}", slice.t());
    <&[i32]>::t2();

    // -----------
    let array = [1.2, 2_f32, 3_f32, 4.2, 5.2];
    println!("{}", array.t());
    <[f32; 5]>::t2();

    // -----------
    let v = vec![String::from("345"), "that".to_string(), "asd".to_owned()];
    println!("{}", v.t());
    <Vec<String>>::t2();

    // -----------
    let p = P;
    println!("{}", p.de());
    println!("{}", P::not_de());
    let o = O;
    println!("{}", o.de());
    println!("{}", O::not_de());

    // associated trait
    println!("Associated Trait");
    let s = ass::Struct::new("fathir".to_owned());
    let t = s.method1();
    dbg!("-->", t);
    let t = s.method2();
    dbg!("-->", t);
}

pub trait MyTrait {
    fn method1(&self) -> i32;
    fn method2(param1: &str) -> Result<(), String>;
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

    fn method2(param1: &str) -> Result<(), String> {
        if param1.is_empty() {
            return Err("empty string".to_string());
        }
        Ok(())
    }

    fn method3(param1: String) {
        println!("do nothing");
    }
}

struct F {
    a: i32,
}

impl F {
    fn accept_trait_from_impl(param: &impl MyTrait) {
        param.method1();
    }

    fn return_trait_from_impl() -> impl MyTrait {
        MyStruct {
            a: 123,
            b: String::from("sdf"),
        }
    }
}

fn return_trait() -> impl MyTrait {
    MyStruct {
        a: 100,
        b: "test".to_string(),
    }
}

fn accept_trait(t: &impl MyTrait) {
    let resp = t.method1();
    dbg!("accept_trait: ", resp);
    let ret = MyStruct::method2("param1");
    dbg!("accept_trait: ", ret);

    MyStruct::method3("nothing inside accept_trait: ".to_string());
}
