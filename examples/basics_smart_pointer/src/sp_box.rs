pub fn do_box() {
    let boxed_i32 = Box::new(123);

    // *T dimana T adalah smart pointer, akan meng-dereferensikan isi dari smart pointer -> T = Box<U> -> *T = U
    dereference(*boxed_i32);

    // &T dimana T adalah smart pointer, akan meng-referensikan isi dari smart pointer -> T = Box<U> -> &T = &U
    reference(&boxed_i32);

    // calling method on box depends on `self` type, since it's call-by-value(`self`) then will automatically got deferred by compiler.
    println!("{}", boxed_i32.pow(2));

    let mut boxed_i32 = Box::new(123);
    println!("before dereference_mut: {boxed_i32}");
    // Karena value dari explicit dereference akan di copy ke dalam parameter, mutability hanya berlaku pada scope fungsi.
    dereference_mut(*boxed_i32);
    println!("after dereference_mut: {boxed_i32}");

    println!("---");

    println!("before reference_mut: {boxed_i32}");
    // &mut T dimana T adalah smart pointer, akan meng-dereferensikan isi dari smart pointer -> T = Box<U> -> &mut T = &U
    reference_mut(&mut boxed_i32);
    println!("after reference_mut: {boxed_i32}");

    let b = Box::new(234);
    let c = &b;
    let d = &c;
    let e: &i32 = &b;
    println!("{:?}", b);
    println!("{:?}", c);
    println!("{:?}", d);
    println!("{:?}", e);

    let f = Box::new(234);
    let g = f.clone();
    println!("{:p}", f);
    println!("{:p}", g);
}

fn dereference(n: i32) {
    println!("from inside dereference: {n}");
}

fn reference(n: &i32) {
    println!("from inside reference: {n}");
}

fn dereference_mut(mut n: i32) {
    n += 1;
}

fn reference_mut(n: &mut i32) {
    *n = *n + 1;
}
