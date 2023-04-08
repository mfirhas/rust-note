#[derive(Debug)]
pub struct Object {
    field1: String,
    field2: i32,
    field3: u64,
}

impl Object {
    pub fn new(field1: String, field2: i32, field3: u64) -> Self {
        Object {
            field1,
            field2,
            field3,
        }
    }

    pub fn method1(&mut self) {
        self.field1.push_str("new str");
    }

    pub fn method2(&mut self) {
        self.field2 += 1;
    }

    pub fn method3(&mut self) {
        self.field3 += 340;
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}
