struct A {
    f: fn(i32) -> i32,
    g: for<'a> fn(&'a i32) -> &'a i32,
}

fn accept_closure<F: Fn(i32) -> i32>(f: F, x: i32) {
    println!("{}", f(x))
}

fn return_closure() -> impl Fn(i32) -> i32 {
    f
}

fn f(x: i32) -> i32 {
    x
}

fn f_lt(x: &i32) -> &i32 {
    x
}

pub fn f_a() {
    let c = |x: i32| -> i32 { x };
    let a = A { f: c, g: f_lt };
    println!("{}", (a.f)(123));
    println!("{}", (a.g)(&234));

    accept_closure(f, 5);
    return_closure()(5);
}
