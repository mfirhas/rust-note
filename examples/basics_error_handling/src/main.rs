use std::{thread::Thread, time::Duration};

fn main() {
    // let arr: [i32; 5] = [1,2,3,4,5];
    // let mut index = 0;
    // loop {
    //     println!("{}: {}", index, arr[index]);
    //     index += 1;
    // std::thread::sleep(Duration::from_secs(1));
    // }
    // match fallible_function() {
    //     Ok(t) => println!("val: {}", t),
    //     Err(e) => panic!("panic {}", e),
    // }

    // match fallible_function() {
    //     Ok(1) => println!("berhasil"),
    //     _ => panic!("panic"),
    // }

    let resp = fallible_function();
    // let ret = resp.expect("number 1");
    // let ret = resp.expect_err("");
    // let ret = resp
    let ret = resp.unwrap_or_else(|x| -> i32 {
        println!("--:: {}", x);
        123
    });
    println!(";;{}", ret);
    let ret = resp.and_then(|x| -> Result<_, &str> {
        println!("sdf {}", x);
        Ok(())
    });
    dbg!(ret);

    let num = 123;
    let ret = matches!(num, 4);
    println!("{}", ret);

    let option = Some(5);
    let ret = matches!(option, Some(5));
    println!("{}", ret);

    let resp = function1();
    println!("chain error: {:?}", resp);

    func2().expect("nganuuuuu");
}

fn fallible_function<'a>() -> Result<i32, &'a str> {
    // some process
    Err("error ninu ninu") // jika gagal
                           // Ok(1) // jika berhasil
}

fn function1() -> Result<String, String> {
    let resp_funciton2 = function2()?;
    // do something with `resp`
    Ok(String::from("yay"))
}

fn function2<'a>() -> Result<i32, &'a str> {
    Err("error in function 2")
}

fn func<'a>() -> Result<String, &'a str> {
    Err("haha")
}

fn func2<'a>() -> Result<String, String> {
    Err("lol".to_string())
}
