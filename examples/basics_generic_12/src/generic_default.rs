use std::fmt::Debug;

#[derive(Debug)]
pub struct Data<T = i32> {
    field: T,
}

impl Data {
    pub fn new() -> Self {
        Data { field: 123 }
    }
}

impl<T: Copy + Debug> Data<T> {
    pub fn new_custom(t: T) -> Self {
        Data { field: t }
    }
}
