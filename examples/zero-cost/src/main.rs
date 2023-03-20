// #[allow(unused)]

// use std::collections::HashMap;
// use tokio::time::Instant;

// use std::{time::Instant as std_instant, borrow::BorrowMut};

// #[tokio::main]
// async fn main() {
//     // // println!("{}", (0..1000).sum::<i8>());
//     // let vec = vec![10000, 200000];
//     // println!("{}", vec.iter().sum::<i32>());

//     // let map = HashMap::from([(1, "anu".to_string()), (2, "nganu".to_string())]);
//     // println!("{:?}", &map);
//     // println!("{:#?}", &map[&1]);
//     // println!("{:#?}", map[&1]);

//     // let c = 234;
//     // let f = move |x| x+c;
//     // println!("{}",f(123));
//     // println!("{:?}", c);

//     // let sd: Vec<i32> = vec![1,2,3,4,5];
//     // let ret = sd.into_iter().map(|x| x*x).collect::<Vec<_>>();
//     // println!("{:?}", ret);

//     // let start = Instant::now();
//     let start = std_instant::now();
//     for i in (0..100000) {
//         nothing().await;
//     }
//     // let end = start.elapsed();
//     let end = start.elapsed();
//     println!("elapsed: {:#?}", end);

//     let c = Cat;
//     static_dispatch(c);
//     do_cat(c);
//     let mut d = Dog;
//     static_dispatch(d);
// }

// async fn nothing() {}

// // sync version
// // fn main() {
// //     let start = std_instant::now();
// //     for i in (0..100000) {
// //         nothing();
// //     }
// //     let end = start.elapsed();
// //     println!("elapsed: {:#?}", end);
// // }

// // fn nothing() {}

// pub trait Animal {
//     fn walk(&self);
//     fn sleep(&self);
//     fn eat(&self);
// }

// fn static_dispatch<A: Animal>(a: A) {
//     a.walk();
//     a.sleep();
//     a.eat();
// }

// fn do_cat<C: Catter>(c: C) {
//     c.walk();
//     c.sleep();
//     c.eat();
//     c.meow();
// }

// trait Catter: Animal {
//     fn meow(&self);
// }
// #[derive(Clone, Copy)]
// struct Cat;

// impl Catter for Cat {
//     fn meow(&self) {
//         println!("cat meowing...")
//     }
// }

// impl Animal for Cat {
//     fn walk(&self) {
//         println!("cat is walking...!")
//     }
//     fn sleep(&self) {
//         println!("cat is sleeping...!")
//     }
//     fn eat(&self) {
//         println!("cat is eating...!")
//     }
// }

// struct Dog;

// impl Animal for Dog {
//     fn walk(&self) {
//         println!("dog is walking...!")
//     }
//     fn sleep(&self) {
//         println!("dog is sleeping...!")
//     }
//     fn eat(&self) {
//         println!("dog is eating...!")
//     }
// }

// pub struct MyStruct<F> {
//     func: F,
// }

// impl<F> MyStruct<F> {
//     pub fn wrap_x<F: FnOnce()>(&mut self, f: F) {
//         self.func = f;
//     }

//     pub fn wrap_y<F: FnMut(i32) -> String>(&mut self, f: F) {
//         self.func = f;
//     }

//     pub fn wrap_z<F: Fn(String)>(&mut self, f: F) {
//         self.func = f;
//     }
// }

use std::fmt::Debug;

#[derive(Debug)]
pub struct MyStruct<F> {
    func: F,
}

impl<F> MyStruct<F> {
    pub fn callback(self) -> F {
        self.func
    }
}

impl<F> MyStruct<F>
where
    F: FnOnce(),
{
    pub fn wrap_x(f: F) -> Self {
        Self { func: f }
    }
}

impl<F> MyStruct<F>
where
    F: FnMut(i32) -> String,
{
    pub fn wrap_y(f: F) -> Self {
        Self { func: f }
    }
}

impl<F> MyStruct<F>
where
    F: Fn() -> String,
{
    pub fn wrap_z(f: F) -> Self {
        Self { func: f }
    }
}

fn main() {
    // let vec = vec![1, 2, 3];
    let sum = (1..=10).reduce(|x, y| x + y).unwrap();
    // assert_eq!(6, sum);
    dbg!(sum);

    let sum = (1..=10).fold(0, |x, y| x + y);
    // assert_eq!(6, sum);
    dbg!(sum);

    let my_struct_x = MyStruct::wrap_x(|| ());
    println!("{:?}", my_struct_x.callback()());
    let mut my_struct_y = MyStruct::wrap_y(|x| x.to_string());
    println!("{:?}", my_struct_y.callback()(123));
    let my_struct_z = MyStruct::wrap_z(|| String::from("anu"));
    println!("{:?}", my_struct_z.callback()());

    dbg!(unsound_ref::<i32>(&123));
    dbg!(unsound_ref::<u128>(&123));
}

// from https://cheats.rs
fn unsound_ref<T>(x: &T) -> &u128 {
    // Signature looks safe to users. Happens to be
    unsafe { std::mem::transmute(x) } // ok if invoked with an &u128, UB for practically anything else
}
