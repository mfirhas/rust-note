#![allow(warnings)]

mod closure_function_pointer;
mod lifetime_st;
mod trait_obj_st;
mod trait_st;
mod variances;

fn main() {
    trait_st::do_trait();
    lifetime_st::lifetime_subtyping();
    closure_function_pointer::f_a();
    trait_obj_st::trait_object();

    fn make_static(_: &'static str) {}
    fn make_a<'a>(_: &'a str) {}

    // let mut s: &'static str = "this";
    // let mut s: &'static str = "this";
    // man(&muqt s);

    // df(s);
    // df(s);

    // mutable invariances
    let a: &str = "this";
    accept_immutable(a);
    // accept_mutable(a); // failed because parameter is mutable and only accept mutable since it's invariant

    // mutable covariance over immutable
    let mut a: &str = "this";
    accept_immutable(a); // works because parameter is immutable and mutable values are covariance over immutable parameters.
    accept_mutable(&mut a);

    // let mutable = immutable
    // let mut a: &mut str;
    // let b: &str = &"anu";
    // a = b;
}

fn accept_immutable(a: &str) {
    // even if a get value from mutable reference, it becomes immutable in this function scope.
}
fn accept_mutable(a: &mut &str) {
    *a = "lskdfm";
}

fn man(a: &'static mut &'static str) {
    // println!("{a}");
}

fn df<'b>(b: &'b str) {}
