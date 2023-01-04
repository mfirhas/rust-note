
mod a;
#[cfg(test)]
mod a_test;

fn main() {
    let s = String::from("lskdmfsdf");
    let a = || {
        println!("{}", s);
        println!("Im printed from closure!");
    };
    a();
    let b = move |x: i32| -> i32 {
        println!("{}", s);
        x * x
    };
    println!("{}",b(3));
    println!("{}", a::add(2, 3));

    let int = 123;
    let cl = move || {
        println!("moved int: {}", int);
    };
    cl();
    println!("==> {}", int);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_inline() {
        assert!(5 == a::add(3, 2));
        assert_eq!(5, a::add(3, 2));
        assert_ne!(2, a::add(3, 2));

        if a::add(3, 2) != 5 {
            panic!("unexpected");
        }
    }
}


mod parent {
    const A: &str = "anu";
    mod child {
        use super::A;
    }
}

mod mod1 {
    pub const B: &str = "nganu";
}

mod mod2 {
    use super::mod1::B;
}

fn im_panic() {
    panic!("OH NO!!!");
}

mod tests_2 {
    use super::*;

    #[test]
    #[should_panic]
    fn test_im_panic() {
        im_panic();
    }
}