use std::fmt::Debug;

fn function<T: Sized>(t: T) {}

struct S<T: ?Sized>(T);

pub fn do_sized() {
    function(123);
    function("123");
    function("123".to_string());
    function(123_f64);
    function('r');

    let arr: [i32] = [2, 3, 4, 5];
    // [i32] has no known size at compile time.
    // use [i32; N] where N is fixed size, or
    // use &[i32] where & indicates it's a reference pointer which size is pointer's size and data referenced from other defined places.
    let s: S<[i32]> = arr;
}
