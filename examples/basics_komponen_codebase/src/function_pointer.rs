pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn f(x: i32) -> fn(i32) -> i32 {
    f2
}

fn f2(x: i32) -> i32 {
    123
}

// failed, because return type is covariance so subtype should be passed to supertype,
// but what happen here is we are passing supertype(closure) into subtype(function pointer)
// pub fn multiply(x: i32, y: i32) -> fn(i32) -> i32 {
//     |z| -> i32 { x * y * z }
// }

pub fn accept_fn(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

pub fn return_fn() -> fn(i32, i32) -> i32 {
    add
}

pub fn return_nested_fn_with() -> fn(i32) -> fn(i32) -> i32 {
    f
}
