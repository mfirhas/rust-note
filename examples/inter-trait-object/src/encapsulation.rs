// encapsulation
#[derive(Debug)]
struct A {
    field1: String,
    field2: i32,
}

impl A {
    fn new() -> Self {
        A {
            field1: String::from("this A"),
            field2: 123,
        }
    }
    fn method_a(&self, n: u64) {
        println!("A: {n}")
    }
}

// kita meng-enkapsulasi A ke dalam B, sehingga A tidak perlu di-ekpose ke user.
#[derive(Debug)]
pub struct B {
    a: A,
    field3: u64,
}

impl B {
    pub fn new() -> Self {
        B {
            a: A::new(),
            field3: 234,
        }
    }
    pub fn method_b(&self) {
        println!("printing B");
        self.a.method_a(self.field3);
    }

    // data A di-enkapsulasi ke dalam B, sehingga bisa di akses tanpa akses ke A.
    pub fn method_b_2(&self) -> (&str, i32) {
        (self.a.field1.as_str(), self.a.field2)
    }
}
