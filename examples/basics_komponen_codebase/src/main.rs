mod function_pointer;
#[allow(unused)]
mod module_a;

use module_a::{
    function, string, CustomResult, Enum, Fff, Int32, Int64, Struct, Trait, CONSTANT_STRING,
    STATIC_STRING,
};

fn main() {
    dbg!(CONSTANT_STRING);
    dbg!(STATIC_STRING);
    dbg!(function("sdf"));
    dbg!(Enum::Tag4 {
        field1: String::from("enum tag 4"),
        field2: 234
    });
    dbg!(Struct::CONSTANT_STRING);
    dbg!(Struct::CONSTANT_I32);
    dbg!(<Struct as Trait>::CONSTANT_STRING);
    dbg!(<Struct as Trait>::CONSTANT_I32);
    // dbg!(Trait::CONSTANT_STRING);
    let s = Struct {
        field1: String::from("test"),
        field2: 400,
    };
    s.method1();

    {
        let mut orphan_reference = &123;
        orphan_reference = &900;
        println!("{:?}", orphan_reference);
        dbg!(orphan_reference);

        // orphan mutable reference without let binding has no lifetime beyond statement declaration.
        // let mut orphan_reference = &mut 123;
        // orphan_reference = &mut 900; // `&mut 900` lifetime only in this assignment statement only.
        // println!("{:?}", orphan_reference);
        // dbg!(orphan_reference);

        let binding: i32 = 123;
        let borrow = &binding;
        dbg!(binding);
        dbg!(borrow);

        let binding: String = "123".to_owned();
        let borrow = &binding;
        println!("{:?}", binding);
        println!("{:?}", borrow);
        dbg!(&binding);
        dbg!(borrow);
    }

    let int32: Int64 = 42;
    dbg!(int32);
    println!("{}", int32);
    let s = "string".to_owned() as string;
    let s: string = "string".to_owned();
    dbg!(&s);
    println!("{:?}", s);

    let ok: CustomResult<i64> = Ok(123);
    // generic assignments inside one if it's field
    let error: CustomResult<_> = Err::<i32, String>("error".to_owned());
    let error: CustomResult<i32> = Err("error".to_owned());

    let fff = Fff::Tag1::<i32, &str, String>(123);
    dbg!(fff);

    let fff = Fff::<i32, &str, String>::Tag1(123);
    dbg!(fff);

    let g = [1, 2, 3, 4];
    dbg!(g.get(5));
    let v = vec![1, 2, 3, 4];
    // dbg!(v[5]); // panic

    println!(
        "function_pointer::accept_fn: {}",
        function_pointer::accept_fn(function_pointer::add, 1, 3)
    );
    println!(
        "function_pointer::return_fn: {}",
        function_pointer::return_fn()(4, 5)
    )

    // let setring: AliasString = "this".to_owned();
    // dbg!(setring);
}
