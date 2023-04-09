use std::fmt::Debug;

// type alias
pub type Int32 = i32;
pub type Int64 = i64;
pub type string = String;

pub type CustomResult<T> = Result<T, String>;

pub const CONSTANT_STRING: &str = "this is string constant";

pub static STATIC_STRING: &str = "this is string static";

// pub type AliasString: Clone = String;

pub fn function(v: &str) -> String {
    let mut a = Struct {
        field1: "sdf".to_owned(),
        field2: 123,
    };
    a.field2 += 1;
    dbg!("-->", a);

    let a = &mut Struct {
        field1: "sdf".to_owned(),
        field2: 123,
    };
    // when we access the field, it automatically dereferenced.
    a.field2 += 1;
    dbg!("-->", a);

    // let a = &mut Struct {
    //     field1: "sdf".to_owned(),
    //     field2: 123,
    // };
    // // because a is immutable, cannot re-assign it
    // a = &mut Struct {
    //     field1: "lskdfm".to_owned(),
    //     field2: 999,
    // };
    // dbg!("-->", a);

    let mut a = &mut Struct {
        field1: "sdf".to_owned(),
        field2: 123,
    };
    // because a is immutable, cannot re-assign it
    // &mut create temporary values dropped at the end of value declarations.
    // a = &mut Struct {
    //     field1: "lskdfm".to_owned(),
    //     field2: 999,
    // };
    // dbg!("-->", a);

    // let mut a = &mut Empty;
    // // `&mut Empty` is orphan reference not binded to anything, hence its lifetime is only in this statement only and dropped after that
    // a = &mut Empty;
    // // `a` will be dropped here
    // dbg!("-->", a);

    let a = &mut 123;
    *a += 34;
    dbg!("-->", a);

    let mut a = 123;
    a = 234;
    dbg!("-->", a);

    // let mut a = &mut 123;
    // // same: orphan reference
    // a = &mut 234;
    // dbg!("-->", a);

    let mut f = &mut 234;
    dbg!(f);

    v.into()
}

#[derive(Debug)]
pub struct Empty;

#[derive(Debug)]
pub struct Struct {
    pub field1: String,
    pub field2: i32,
}

#[derive(Debug)]
pub enum Fff<T, U, V> {
    Tag1(T),
    Tag2 { field: U },
    Tag3(V),
}

pub trait Trait {
    const CONSTANT_STRING: &'static str = "trait constant";
    const CONSTANT_I32: i32;
    // cannot declare static inside `trait`
    // static STATIC_STRING: &str = "trait static";
    type Item: Sized + Clone + Debug;

    fn method1(&self) -> Self::Item;
    fn default_method() -> i32 {
        123
    }
}

impl Struct {
    pub const CONSTANT_STRING: &str = "struct constant";
    pub const CONSTANT_I32: i32 = 123;
    // cannot declare static inside `impl`
    // pub static STATIC_STRING: &str = "static";
    pub fn method1() -> String {
        String::new()
    }
    fn method2() -> i32 {
        123
    }
}

impl Trait for Struct {
    const CONSTANT_STRING: &'static str = "Struct overwrite Trait's constant";
    const CONSTANT_I32: i32 = 99999;
    type Item = String;
    fn method1(&self) -> Self::Item {
        self.field1.clone()
    }
}

#[derive(Debug)]
pub enum Enum {
    Tag1,
    Tag2(String),
    Tag3(i32, f32),
    Tag4 { field1: String, field2: i32 },
    Tag5(Struct),
}
