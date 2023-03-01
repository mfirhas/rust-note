use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
};

fn main() {
    let list: Vec<i32> = vec![1, 2, 3, 3];
    let list_iterator = list.iter();
    let new_list = list_iterator
        .map(|x| (x * x) as i64)
        .collect::<HashSet<_>>();
    dbg!("before:", &list);
    dbg!("after:", &new_list);

    let sl = [1, 2, 3, 4, 5];
    sl.cetak();

    let df = "123";
    let parsed_df = df.parse().unwrap_or(0);
    let closure = |x| x * x;
    println!("{:?}", closure(123));

    let input = 123;
    println!(
        "Identity of {input:?} --> {:?}",
        std::convert::identity(input)
    );
    let input = Some(5);
    println!(
        "Identity of {input:?} --> {:?}",
        std::convert::identity(input)
    );
    let input = "anu";
    println!(
        "Identity of {input:?} --> {:?}",
        std::convert::identity(input)
    );
    let input = "";
    println!(
        "Identity of {input:?} --> {:?}",
        std::convert::identity(input)
    );

    let input: Option<i32> = None;
    println!(
        "Identity of {input:?} --> {:?}",
        std::convert::identity(input)
    );

    let input = vec![Some(4), None, Some(100)];
    println!(
        "Identity of {:?} --> {:?}",
        input.clone(),
        input
            .into_iter()
            .filter_map(std::convert::identity)
            .collect::<Vec<_>>()
    );

    let arr = &[1, 2, 3];
    let mut chunks = arr.chunks(2);
    println!("{:?}", chunks.next());
    println!("{:?}", chunks.next());

    let b = true;
    let a = b.then(|| {
        println!("nganu");
        123
    });

    let tuples = [("one", 1), ("one", 5), ("two", 2), ("three", 3)];
    let m: HashMap<_, _> = tuples.into_iter().collect();
    println!("{:?}", m);

    let vec_tuples = vec![(1, 1), (2, 2), (3, 3)];
    let m: HashMap<_, _> = vec_tuples.into_iter().collect();
    println!("{:?}", m);
}

trait A {
    fn cetak(self);
}

impl<T> A for T
where
    T: IntoIterator + Debug,
{
    fn cetak(self) {
        println!("{:?}", self);
    }
}
