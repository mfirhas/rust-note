fn main() {
    let string = String::from("string");
    let s = StructWithLifetime {
        string: string.as_str(),
    };
    println!("{:?}", s);
    // take(string);

    println!("-----------");

    let mut sdf = StructWithLifetime { string: "val" };
    {
        let string = String::from("from this string");
        sdf = StructWithLifetime {
            string: string.as_str(),
        };
    }
    // println!("{:?}", sdf);

    println!("-----------");

    let mut string: String = String::from("anu");
    let s: StructWithLifetime;
    {
        string = String::from("anu--modified");
    }
    let s = StructWithLifetime {
        string: string.as_str(),
    };
    println!("--> {:?}", s);

    let s;
    let df = String::from("lskdmf");
    {
        s = "anu";
        println!("{}", s);
        println!("{}", df);
    };
    println!("{}", s);
    println!("--> {}", df);

    // -----------------------------

    let name = "anu_name";
    let svc = Service { name };
    {
        let resp = svc.method1();
        println!("method1--> {}", resp);
    }
    println!("method2--> {}", svc.method2());
    println!("method3--> {}", svc.method3());

    let static_str: &'static str = "live forever";
    println!("{static_str}");

    println!("{:#?}", multiple_lifetime("lskdmf"));
}

#[derive(Debug)]
struct StructWithLifetime<'a> {
    string: &'a str,
}

// fn initialize(s: String) {
//     s = String::from("anu");
// }

fn take(s: String) {
    println!("im taken: {}", s)
}

struct Service<'a> {
    name: &'a str,
}

impl<'a> Service<'a> {
    pub fn method1(&'a self) -> &str {
        f(self.name);
        self.name
    }
}

impl<'a> Service<'_> {
    pub fn method2(&self) -> &str {
        self.name
    }
}

impl Service<'_> {
    pub fn method3(&self) -> &str {
        self.name
    }
}

fn f<'a>(s: &'a str) -> &'a str {
    println!("f() -> {}", s);
    s
}

struct Service2 {
    name: String,
}

impl<'a> Service2 {
    pub fn method1(s: &'a str) -> &str {
        println!("{}", s);
        s
    }
}

fn multiple_lifetime(inp: &str) -> (&str, &str, &str) {
    (inp, inp, inp)
}
