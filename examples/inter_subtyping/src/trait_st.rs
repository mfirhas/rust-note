/// subtyping in rust, replacing inheritance.

// trait subtyping
pub trait Base {
    fn walk(&self);
}

pub trait Child: Base {
    fn run(&self);
    fn fly(&self);
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct B {
    name: String,
}
impl B {
    pub fn new(name: &str) -> Self {
        B {
            name: String::from(name),
        }
    }
}
impl Base for B {
    fn walk(&self) {}
}

fn process<T: Base>(b: T) {
    println!("process called");
    b.walk();
}

fn process_return_base() -> impl Base {
    B::new("anu")
}

fn process_sub<C: Child>(c: C) {
    println!("process_sub called");
    c.run();
    c.fly();
}

pub fn do_trait() {
    let a = A {
        name: "this A".to_string(),
    };
    process(a.clone());
    process_sub(a);

    let b = B {
        name: "this B".to_string(),
    };
    process(b.clone());
    // process_sub(b); // failed, because process_sub require Child trait, subtype of Base trait, but B only implement Base trait.
    let base = process_return_base();
}
