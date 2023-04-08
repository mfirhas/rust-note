use std::fmt::Debug;

/// composition
///
///
pub trait IDep: Debug {}
pub trait Field: Debug {}
#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}

#[derive(Debug)]
pub struct Struct {
    field1: Box<dyn IDep>,
    field2: Vec<Box<dyn Field>>,
}

#[derive(Debug)]
pub struct Dep;
impl IDep for Dep {}

impl Struct {
    pub fn new() -> Self {
        let mut v: Vec<Box<dyn Field>> = Vec::new();
        v.push(Box::new(123));
        v.push(Box::new("anu"));
        v.push(Box::new(String::from("nmnm")));
        v.push(Box::new(Custom));
        Self {
            field1: Box::new(Dep),
            field2: v,
        }
    }
}

/// subtyping in rust, replacing inheritance.

// trait subtyping
pub trait Base {
    fn walk(&self);
}

pub trait Child: Base {
    fn run(&self);
    fn fly(&self);
}

pub struct A {
    name: String,
}

impl A {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Base for A {
    fn walk(&self) {}
}

impl Child for A {
    fn run(&self) {}
    fn fly(&self) {}
}

pub fn process<T: Base>(b: T) {
    b.walk();
}

// lifetime subtyping
pub fn lifetime_subtyping() {
    // 'b
    let b = String::from("lifetime b"); // lifetime of b is 'b
    let c;
    let d;
    {
        // 'a
        let a = String::from("lifetime a"); // lifetime of a is 'a
        c = lifetime_b(&a, &b);
        // error, because c takes lifetime of a which is the current lifetime, while c has type &'b str, and 'a is not subtype of 'b, instead 'b is subtype of 'a
        // because b live longer than a, b outlives a.
        // c = lifetime_a(&a, &b);
        println!("{c}");

        // d get value of lifetime 'a, which is the current lifetime
        // while lifetime of d declared above is 'b.
        // this works because lifetime is covariant each other.
        // means that lifetime can be substituted with its variances.
        // d is declared inside lifetime 'b, and 'b is subtype of 'a, hence 'a and 'b have relations eachother that 'b extends 'a lifetime.
        // based on this, d will have shorter lifetime than it was declared above, hence cannot live beyond this scope.
        d = lifetime_a(&a, &b);
        println!("{d}");
    }
    println!("{c}"); // can still access c, because c lifetime is 'b, and value of c returned from lifetime_b() takes lifetime of b, and b is beyond a.
                     // println!("{d}"); // will error because d lifetime has been shortened to 'a inside above scope.
}

// 'b: 'a means b lifetime outlives a, means b live longer than a, means b should live at least as a, means a must die first then b.
fn lifetime_b<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'b str {
    b
}

fn lifetime_a<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
    a
}
