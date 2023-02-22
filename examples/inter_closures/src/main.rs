mod a;

use a::{MyTrait, MyTrait2};

fn main() {
    let int = 123;
    let closure = || {
        let x = 123;
        println!("{}", x);
    };
    closure();
    println!("{}", int);

    let mut s = String::from("lkmlkm");
    let a = &mut s;
    let b = <&mut String as MyTrait>::method(&a);
    let c = <String as MyTrait2>::method(&b);
    let d = 123.method();
    println!("{}",b);
    println!("{}",c);
    println!("{}",d);

    println!("--------------");

    // FnOnce
    println!("---------FnOnce");
    let s = "sdf".to_owned();
    do_fn_once(|| {
        println!("im fn_once {:?}", s);
    });
    println!("im fn_once {:?}", s);
    println!("---------");

    // FnMut
    let mut mutate_me = String::from("start");
    println!("----------FnMut");
    do_fn_mut(|| {
        mutate_me.push_str("-end");
        println!("do_fn_mut");
    });
    println!("{}", mutate_me); // will print "start-end-end-end", because closure invoked 3 times inside `do_fn_mut`
    println!("----------");

    // Fn
    println!("----------Fn");
    let reference_me = "do_fn".to_owned();
    do_fn(|| {
        println!("im moved: **{}", reference_me);
    });
    // println!("{}", reference_me);
    println!("----------");

}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn do_fn_once<F: FnOnce()>(f: F) {
    f();
    f();
}

fn do_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
    f();
}

fn do_fn<F: Fn()>(f: F) {
    f();
    f();
    f();
}

