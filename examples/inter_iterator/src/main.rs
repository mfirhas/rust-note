#[allow(unused)]
mod a;
use a::A;
use std::{
    collections::{HashMap, HashSet, LinkedList},
    fmt::Debug,
    str::FromStr,
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

    let mut mut_me = String::from("from_this");
    let fnmut = || {
        let mut string = mut_me.clone();
        // mut_me.push_str("add this");
        string.push_str("add this");
        println!("{:?}", string);
    };
    func(fnmut);
    println!("{:?}", mut_me);

    println!("***************");
    let mut i = 0;
    loop {
        match i {
            2 => break println!("Yes"),
            _ => println!("No"),
        };
        i += 1;
    }

    println!("{}", fullname("Muhammad")("Fathir")("Irhas"));
    println!("{}", fullname2("Muhammad")("Fathir")("Irhas")("S.Kom"));

    let mut a = 20;
    accept_mut(&mut a);
    println!("{}", a);

    let v = vec![1, 2, 3, 4];
    let v_doubles = v.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", v_doubles);

    let opt: Option<i32> = None; //Some(5);
    let ret = opt.map(|x| x * x).unwrap_or(Default::default());
    println!("{}", ret);

    let ret = functor(123, |x| x * x);
    println!("{ret}");

    let func = |x| {
        move |y| {
            move |z| {
                move |a| {
                    // println!("{a}");
                    (x * y * z) + a
                }
            }
        }
    };
    println!("{:?}", func(3)(9)(2)(2));

    let fullname = |firstname| {
        move |middlename| move |lastname| format!("{} {} {}", firstname, middlename, lastname)
    };

    println!("{}", fullname("Muhammad")("Fathir")("Irhas"));

    println!("{}", currying1(1)(2));
    println!("{}", currying3(1)(2)(3));

    println!("monad ---");
    let opsi: Option<&str> = Some("test");
    let ret = opsi.map(|x| x); //.or(None);
    println!("{:?}", ret);
    let ret = opsi.and_then(|x| Some(123)); //.or(None);
    println!("{:?}", ret);
    let ret = opsi.into_iter().map(|x| x).collect::<Vec<_>>();
    println!("{:?}", ret);

    let opsi = Some(["test"]);
    let ret = opsi.into_iter().map(|x| x).collect::<Vec<_>>();
    println!("{:?}", ret);

    let vec = vec![1, 2, 3];
    let ret = vec.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("{:?}", ret);
    let ret = vec.iter().map(|x| x * x).collect::<LinkedList<_>>();
    println!("{:?}", ret);

    let vec = [(1, "1"), (2, "2"), (3, "3")];
    let ret = vec.into_iter().map(|x| x).collect::<HashMap<i32, &str>>();
    println!("{:#?}", ret);
    // failed because HashMap need to take ownership of iterator elements.
    // hence HashMap cannot be built from &(K,V) because it's not implemented for FromIterator(&(K,V))
    // let ret = vec.iter().map(|x| x).collect::<HashMap<i32, &str>>();
    // println!("{:#?}", ret);
    let tuple = [(1, 1), (2, 2), (3, 3)];
    let ret = tuple.iter().map(|x| *x).collect::<HashMap<i32, i32>>();
    println!("{:#?}", ret);
    let tuple = [
        ("1".to_string(), "1".to_string()),
        ("2".to_string(), "2".to_string()),
        ("3".to_string(), "3".to_string()),
    ];
    let ret = tuple
        .iter()
        .map(|x| x.clone())
        .collect::<HashMap<String, String>>();
    println!("tupe string: {:#?}", ret);

    let tuple = [
        ("1".to_string(), "1".to_string()),
        ("2".to_string(), "2".to_string()),
        ("3".to_string(), "3".to_string()),
    ];
    let ret = tuple
        .into_iter()
        .map(|x| x)
        .collect::<HashMap<String, String>>();
    println!("tupe string: {:#?}", ret);

    let tuple = [(1, 1), (2, 2), (3, 3)];
    let ret = tuple
        .iter()
        .map(|x| (x.0, x.1))
        .collect::<HashMap<i32, i32>>();
    println!("{:#?}", ret);

    let map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
    let ret = map.into_iter().map(|x| x).collect::<Vec<_>>();
    println!("{:#?}", ret);
}

fn functor<T, F>(t: T, f: F) -> T
where
    T: Sized + Debug,
    F: Fn(T) -> T,
{
    println!("{t:?}");
    f(t)
}

fn accept_mut(a: &i32) {
    // *a = *a + 10;
    println!("-+-+> {}", a);
}

fn currying1(first: i32) -> impl Fn(i32) -> i32 {
    move |second| first * second
}

// failed because it against the rule of `impl Trait` placement.
// fn currying2(first: i32) -> impl Fn(i32) -> impl Fn(i32) -> i32 {
//     move |second| move |third| first * second * third
// }

fn currying3(first: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    move |second| Box::new(move |third| first * second * third)
}

type Function<'a> = Box<dyn Fn(&'a str) -> String + 'a>;

fn fullname<'a>(firstname: &'a str) -> impl Fn(&'a str) -> Function<'a> {
    move |middlename| Box::new(move |lastname| format!("{} {} {}", firstname, middlename, lastname))
}

type Function2<'a> = Box<dyn Fn(&'a str) -> Function + 'a>;

fn fullname2<'a>(firstname: &'a str) -> impl Fn(&'a str) -> Function2<'a> {
    move |middlename| {
        Box::new(move |lastname| {
            Box::new(move |gelar| format!("{} {} {}, {}", firstname, middlename, lastname, gelar))
        })
    }
}

// fn ff() -> impl Fn() -> impl Fn() {}

fn func<F>(f: F)
where
    F: Fn(),
{
    f();
}
