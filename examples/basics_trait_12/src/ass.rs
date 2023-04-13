use std::fmt::Debug;

pub trait TraitAss {
    type Item: Sized + Debug + Default;

    fn method1(&self) -> &Self::Item;

    fn method2(&self) -> (Self::Item, i32) {
        (Default::default(), 0)
    }
}

pub struct Struct<T> {
    field1: i32,
    field2: T,
}
impl<T> Struct<T> {
    pub fn new(name: T) -> Self {
        Self {
            field1: 123,
            field2: name,
        }
    }
}

impl<T> TraitAss for Struct<T>
where
    T: Sized + Debug + Default,
{
    type Item = T;

    fn method1(&self) -> &Self::Item {
        &self.field2
    }
}
