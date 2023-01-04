fn main() {
    let anu = "anu";
    {
        let s = String::from(anu);
        println!("anu -> {}", anu);
        // string s will be dropped here at the end of the scope
    }

    println!("{}", anu);
    let d = "lksmdf";
    let f = d.clone();
    println!("{}", d);
    println!("{}", f);

    println!("Hello, world!");

    // let s = String::from("hooh");
    // function(s);
    // println!("{}", &s);
    // println!("{}", s);

    borrow();
    mutable_borrow();

    let x = "xx";
    let y = "yyy";
    let z = longest(x, y);
    println!("{}", z);

    let ret = asd();
    println!("{}", ret);

    let ret = fff("asd", "dfv");
    println!("ret {}", ret);

    // let string = "string";
    // let s = StructWithLifetime{
    //     string,
    // };
    // println!("{}", s);

    println!("--> {}", A);
}

static A: &str = "anu";

fn function(s: String) {}

fn borrow() {
    let anu = "anu";
    {
        println!("{}", anu); // move scope
    }

    println!("{}", anu); // still valid here
    accept_borrow(anu);
    println!("{}", anu); // still valid here
}

fn accept_borrow(s: &str) {
    println!("{}", s);
}

fn mutable_borrow() {
    let mut a = 123;

    let b = &mut a;
    // let c = &mut a;

    // {
    //     let d = &mut a;
    // }

    println!("{}", b);
    // println!("{}", c);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn asd() -> &'static str {
    "anu"
}

fn fff<'a>(x: &str, y: &str) -> &'a str {
    let ret = "anu";
    ret
}
// fn fff2<'a>(x: &str, y: &str) -> &'a str {
//     let ret = String::from("anu");
//     ret.as_str()
// }