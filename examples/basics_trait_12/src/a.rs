pub trait Trait {
    fn t(&self) -> i32;
    fn t2();
}

pub struct Struct {
    pub s: String,
}

impl Trait for Struct {
    fn t(&self) -> i32 {
        println!("{}", self.s);
        123
    }
    fn t2() {}
}

pub enum Enum {
    F,
    G,
    H(String),
}

impl Trait for Enum {
    fn t(&self) -> i32 {
        match &*self {
            Enum::H(s) => {
                println!("{}", s);
            }
            _ => {
                println!("nothing");
            }
        }
        123
    }
    fn t2() {}
}

impl Trait for i32 {
    fn t(&self) -> i32 {
        *self
    }
    fn t2() {
        println!("i32::t2");
    }
}

impl Trait for &[i32] {
    fn t(&self) -> i32 {
        *self.get(0).unwrap_or(&123)
    }
    fn t2() {
        println!("&[i32]::t2");
    }
}

impl Trait for [f32; 5] {
    fn t(&self) -> i32 {
        if !self.is_empty() {
            return *self.get(0).unwrap() as i32;
        }
        123
    }

    fn t2() {
        println!("[f32;5]::t2");
    }
}

impl Trait for Vec<String> {
    fn t(&self) -> i32 {
        if !self.is_empty() {
            let parsed_string = self.get(0).unwrap().parse::<i32>();
            if parsed_string.is_err() {
                return 0;
            }
            return parsed_string.unwrap();
        }
        123
    }

    fn t2() {
        println!("Vec<String>::t2");
    }
}
