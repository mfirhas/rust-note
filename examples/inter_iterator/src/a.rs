use std::fmt::Debug;

pub trait A {
    fn cetak(self);
}

impl<T> A for T
where
    T: IntoIterator + Debug,
{
    fn cetak(self) {
        println!("cetak donk: {:?}", self);
    }
}
