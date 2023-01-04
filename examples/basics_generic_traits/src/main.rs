use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

fn main() {
    function_generic_bound(123);
    function_generic_multiple_bounds::<&str>("this is testing");
    function_multiple_generic_multiple_bounds::<&str, i64>("test", 123);
    function_generic_multiple_bounds("this is testing");
    function_multiple_generic_multiple_bounds("test", 123);
    let resp: String = function_generic_trait_bounds_where("test", 123, "this");
    println!("{}", resp);

    let ms = MyStruct {
        field1: 123,
        field2: "test",
    };
    println!("{:?}", ms);

    let ms = MyStruct3 {
        field1: 123,
        field2: "sdf",
    };
    MyStruct3::method1("test", 123.3_f32);
    ms.method2();
    MyStruct3::<bool, i32>::method1(true, 900);

    MyEnum::<i32, f32>::Value1(9999).method2();

    MyStruct4::<String, i32>::method1(123);
    let my_struct4 = MyStruct4 {
        field1: String::from("this"),
        field2: 234,
    };
    my_struct4.method2(String::from("lskdmf"));
    <MyStruct4<String, i32> as MyTrait<i32, String>>::method1(4999);
    let resp = my_struct4.method3(7890);
    println!("{}", resp);
}

fn function_generic_bound<T: Display>(param: T) {
    println!("{}", param);
}

fn function_generic_multiple_bounds<T: Display + Clone>(param: T) {
    println!("{}", param.clone());
}

fn function_multiple_generic_multiple_bounds<T: Display + Clone, U: Debug>(param1: T, param2: U) {
    println!("{} -> {:?}", param1.clone(), param2)
}

fn function_generic_trait_bounds_where<T, U, V>(param1: T, param2: U, param3: V) -> String
where
    T: Clone + Display,
    U: Debug,
    V: Display,
    String: From<V>,
{
    println!("{} {:?}", param1, param2);
    String::from(param3)
}

// generic in struct
#[derive(Debug)]
struct MyStruct<T: Display, U: Debug + Display> {
    field1: T,
    field2: U,
}

struct MyStruct3<T, U> {
    field1: T,
    field2: U,
}

impl<T: Display, U: Debug + Display> MyStruct3<T, U> {
    pub fn method1(param1: T, param2: U) {
        println!("{}, {}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{}, {}", self.field1, self.field2);
    }
}

#[derive(Debug)]
pub enum MyEnum<T, U> {
    Value1(T),
    Value2(U),
}

impl<T: Debug + Display, U: Debug + Display> MyEnum<T, U> {
    pub fn method1(param1: T, param2: U) {
        println!("{:?}, {:?}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{:?}", self);
    }
}

trait MyTrait<T: Display, U: Display + Clone> {
    fn method1(param: T);
    fn method2(&self, param: U);
    fn method3(&self, param: T) -> U;
}

struct MyStruct4<T, U> {
    field1: T,
    field2: U,
}

// impl<T: Display, U: Clone + Display> MyTrait<i32, String> for MyStruct4<T, U> {
//     fn method1(param: i32) {
//         println!("{}", param);
//     }

//     fn method2(&self, param: String) {
//         println!("{} : {}", self.field2, param.clone());
//     }

//     fn method3(&self, param: i32) -> String {
//         format!("field1: {}, field2: {}, param: {}", self.field1, self.field2, param)
//     }
// }

impl<T, U> MyTrait<i32, String> for MyStruct4<T, U>
where
    T: Display,
    U: Clone + Display,
{
    fn method1(param: i32) {
        println!("{}", param);
    }

    fn method2(&self, param: String) {
        println!("{} : {}", self.field2, param.clone());
    }

    fn method3(&self, param: i32) -> String {
        format!(
            "field1: {}, field2: {}, param: {}",
            self.field1, self.field2, param
        )
    }
}
