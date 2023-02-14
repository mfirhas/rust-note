#[allow(unused)]
fn main() {
    let loop_duration = 20;
    let a = 1;
    if a == 1 {
        println!("True");
    }

    if a == 2 {
        println!("True");
    } else {
        println!("False");
    }

    let a = 3;
    if a == 2 {
        println!("True");
    } else if a == 3 {
        println!("True again");
    } else {
        println!("False");
    }

    let f = if a % 2 == 0 {
        String::from("a: {a} is even")
    } else {
        String::from("a: {a} is odd")
    };
    println!("--> {f:?}");

    let mut c = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(loop_duration));
        if c == 10 {
            println!("Done");
            break;
        }
        if c % 2 == 0 {
            println!("c: {c} even number");
            c += 1;
            continue;
        } else if c % 2 != 0 {
            println!("c: {c} odd number");
            c += 1;
            continue;
        }
    }

    let mut g = 0;
    let ret = loop {
        g += 1;
        if g == 10 {
            break g * 50; // 500
        }
    };
    println!("---> ret: {ret}");

    println!("-----------------");
    let mut d = 0;
    'outer: loop {
        d += 1;
        'inner: loop {
            std::thread::sleep(std::time::Duration::from_millis(loop_duration));
            if d == 10 {
                break 'outer; // no label means break current label
            }
            if d % 2 == 0 {
                println!("d: {d} even number");
                continue 'outer;
            } else {
                println!("d: {d} odd number");
                d += 1;
                continue; // no label means continue current label ('inner)
            }
        }
    }

    let mut a = 0;
    while a < 10 {
        println!("while loop: {a}");
        a += 1;
    }

    for i in 0..10 {
        println!("i: {i}");
    }

    println!("---");

    for i in 0..=10 {
        println!("i: {i}");
    }

    let ff = 1..=10;
    println!("{:#?}", ff);

    // error because b doesn't implement Interator trait
    // let b = 123;
    // for x in b {
    //     println!("{}",x);
    // }

    // into_iter()
    let data1 = [1, 2, 3, 4, 5];
    println!("into_iter():");
    /**
     * Since data type is array and elements are primitive types(i32),
     * Using into_iter() on it will only copy it since no moving required.
     * Hence you can still call data after these 2 iterator since "move" mechanism for copy data(stack allocated) is only "copy".
     */
    for x in data1 {
        // `into_iter()` will be applied by default by compiler if not specified
        println!("{x}");
    }
    // kode di atas sama dengan
    for x in data1.into_iter() {
        println!("{x}");
    }

    // getting index and value
    for (i, x) in data1.into_iter().enumerate() {
        println!("{i}:{x}");
    }
    println!("data: {data1:?}");

    /**
     * if you use into_iter() to data with borrowed type(reference), then into_iter() cannot own the elements since its parent is referenced/borrowed.
     * Hence in each iterator you only see the element as &i32.
     */
    let data1: &[i32] = &[1, 2, 3, 4, 5];
    for x in data1.into_iter() {
        println!("{x}");
    }
    println!("{data1:?}");

    let data1 = &[1, 2, 3, 4, 5];
    for x in data1.into_iter() {
        println!("{x}");
    }
    println!("{data1:?}");

    /**
     * data1 is vector type which is owned type. So into_iter() will make it moved.
     */
    let data1 = vec![1, 2, 3, 4, 5];
    for x in data1 {
        // implicitly and defaultly implement into_iter()
        println!("{}", x);
    }
    // println!("data: {data1:?}"); //// won't work because data already moved by into_iter()
    let data1_1 = vec![1, 2, 3, 4, 5];
    for x in data1_1.into_iter().enumerate() {
        println!("{} {}", x.0, x.1);
    }

    // iter()
    let data2 = [1, 2, 3, 4, 5];
    println!("iter():");
    /**
     * Data type is array and calling iter() method in interation making it borrow the items inside array `&i32`.
     * No copy happen since it's referencing data from array.
     */
    for x in data2.iter() {
        println!("{x}");
    }
    println!("data: {data2:?}");

    // getting index and value
    for x in data2.iter().enumerate() {
        println!("{} {}", x.0, x.1);
    }

    /**
     * vector is owned type but iter() only borrow the items inside so can be used after iteration.
     */
    let data2 = vec![1, 2, 3, 4, 5];
    for x in data2.iter() {
        println!("{x}");
    }
    println!("data: {data2:?}");

    // iter_mut()
    let mut data3 = [1, 2, 3, 4, 5];
    /**
     * iter_mut() will borrow each items as mutable reference if the source data is mutable.
     */
    for x in data3.iter_mut() {
        *x = *x * 2;
    }
    println!("data: {data3:?}");

    let mut data3 = vec![1, 2, 3, 4, 5];
    /**
     * When you iter_mut and enumerate the data, you can only mutate the data's value, not index.
     * Also it won't take any ownership since it's borrowed/referenced.
     */
    for (i, x) in data3.iter_mut().enumerate() {
        *x = *x * 5;
        println!("{i} {x}");
    }
    println!("data: {data3:#?}");

    let number = 19;

    println!("Tell me about {}", number);
    /**
     * If we want to match against i32,
     * there'll be 2147483647 possible arms that we have to specify inside match.
     * This is impossible to write by hand,
     * most of the time you only need certain values to be checked against,
     * hence you can use `_` to handle the rest of the possible arms.
     */
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("SUDAH TUA!!!!"),
    }

    let tuple = (1, 2, 3);
    match tuple {
        (2, y, z) => println!("1st arm"), // invalidated because first tuple element must be 1 in order to match.
        (x, 5, z) => println!("2nd arm"), // invalidated because second element must be 2 in order to match.
        (x, y, 3) => println!("3rd arm"), // validated because 3rd matched, 1st and 2nd arm only fetched as variable(can be anything from the tuple itself).
        _ => println!("404"),
    }

    // from https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring/destructure_slice.html
    // Try changing the values in the array, or make it a slice!
    let array = [1, -2, 6];

    match array {
        // Binds the second and the third elements to the respective variables
        [0, second, third] => println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),

        // Single values can be ignored with _
        [1, _, third] => println!(
            "array[0] = 1, array[2] = {} and array[1] was ignored",
            third
        ),

        // You can also bind some and ignore the rest
        [-1, second, ..] => println!(
            "array[0] = -1, array[1] = {} and all the other ones were ignored",
            second
        ),
        // The code below would not compile
        // [-1, second] => ...

        // Or store them in another array/slice (the type depends on
        // that of the value that is being matched against)
        [3, second, tail @ ..] => println!(
            "array[0] = 3, array[1] = {} and the other elements were {:?}",
            second, tail
        ),

        // Combining these patterns, we can, for example, bind the first and
        // last values, and store the rest of them in a single array
        [first, middle @ .., last] => println!(
            "array[0] = {}, middle = {:?}, array[2] = {}",
            first, middle, last
        ),
    }

    // `allow` required to silence warnings because only
    // one variant is used.
    #[allow(dead_code)]
    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let color = Color::RGB(122, 17, 40);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
        // Don't need another arm because all variants have been examined
    }

    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let reference = &4;

    match reference {
        val => println!("value: {val:?}"),
    }
    match reference {
        &4 => println!("reference of four"),
        _ => println!("other"),
    }
    match reference {
        4 => println!("value of four"),
        _ => println!("other"),
    }
    match reference {
        // If `reference` is pattern matched against `&val`, it results
        // in a comparison like:
        // `&i32`
        // `&val`
        // ^ We see that if the matching `&`s are dropped, then the `i32`
        // should be assigned to `val`.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    // To avoid the `&`, you dereference before matching.
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        }
    }

    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // you can destructure structs and rename the variables,
        // the order is not important
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // and you can also ignore some variables:
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y),
    }

    // from: https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html
    enum Temperature {
        Celsius(i32),
        Fahrenheit(i32),
    }

    let temperature = Temperature::Celsius(35);

    match temperature {
        // guard harus ditaruh lebih awal dibanding tanpa guard karena lebih spesifik.
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(t) => println!("{}C is below 30 Celsius", t),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
    }

    match temperature {
        // guard harus ditaruh lebih awal dibanding tanpa guard karena lebih spesifik.
        Temperature::Celsius(t) if t > 30 => println!("{}C is above 30 Celsius", t),
        // The `if condition` part ^ is a guard
        Temperature::Celsius(20) => println!("C is below 30 Celsius"),

        Temperature::Fahrenheit(t) if t > 86 => println!("{}F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}F is below 86 Fahrenheit", t),
        _ => println!("404"), // wildcard dibutuhkan karena arm ke-dua memiliki value langsung yang menghilangkan sifat exhautiveness dari arms.
    }

    let number: i8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        i if i < 0 => println!("Lesser than zero"),
        _ => println!("404"), // wildcard tetap harus ada karena guards tidak memvalidasi sifat exhaustive dari arms.
    }

    let number: i8 = 4;
    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        i if i < 0 => println!("Lesser than zero"),
        i => println!("{i}"),
        // wildcard tidak dibutuhkan karena exhaustiveness sudah di-achieve oleh arm `i => println!("{i}")` karena sudah nge-cover apa saja.
    }

    // A function `age` which returns a `u32`.
    fn age() -> u32 {
        15
    }

    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // Could `match` 1 ..= 12 directly but then what age
        // would the child be? Instead, bind to `n` for the
        // sequence of 1 ..= 12. Now the age can be reported.
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        // Nothing bound. Return the result.
        n => println!("I'm an old person of age {:?}", n),
    }

    fn some_number() -> Option<u32> {
        Some(42)
    }

    match some_number() {
        // Got `Some` variant, match if its value, bound to `n`,
        // is equal to 42.
        Some(n @ 42) => println!("The Answer: {}!", n),
        // Match any other number.
        Some(n) => println!("Not interesting... {}", n),
        // Match anything else (`None` variant).
        _ => (),
    }

    let s = Some(5);
    if let Some(n) = s {
        println!("{n}");
    }
    if let Some(3) = s {
        println!("matched!");
    } else {
        println!("not matched!");
    }

    if let n = s {
        println!("{:#?}", n);
    }

    let astruct = A {
        a: 123,
        b: String::from("anu"),
    };
    let bb = String::from("anu");
    if let A { a: 123, b: bb } = astruct {
        println!("matched$$");
    }

    let mut a = Some(5);
    while let Some(n) = a {
        a = Some(n + 1);
        if n > 10 {
            a = None;
        }
        println!("$");
        std::thread::sleep(std::time::Duration::from_millis(loop_duration));
    }
    println!("DONE while let");

    let mut s: &str = "lskdmfsdf";
    s = "asdasd";
    println!("{s}");
    let mut ss: String = s.into();
    ss.push('p');
    println!("{ss}");
}

struct A {
    pub a: i32,
    pub b: String,
}
