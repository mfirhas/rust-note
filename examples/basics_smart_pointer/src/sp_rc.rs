use std::rc::Rc;

#[derive(Debug)]
pub struct Data;

pub fn do_rc() {
    let data = Data;
    dbg!(data); // dbg! macro move ownership of non copyable value
                // dbg!(data); // can't call debug data here since already moved into dbg!(data)

    let shared_data = Rc::new(Data);
    let borrow_shared_data = Rc::clone(&shared_data);
    let c = &borrow_shared_data;
    let d: &Rc<_> = &borrow_shared_data;
    let e: &Rc<Data> = &borrow_shared_data;
    let f: &Data = d; // &Rc<T> get coerced into &T
    println!("c - d - e - f -> {:?} - {:?} - {:?} - {:?}", c, d, e, f);

    let count = Rc::strong_count(&shared_data);
    println!("strong count: {count}");
    // reference counts: 1. shared_data, and 2. borrow_shared_data
    assert_eq!(2, count);
    {
        let inner_scope_borrow = Rc::clone(&borrow_shared_data);
        let inner_count = Rc::strong_count(&shared_data);
        // reference counts: 1. shared_data, 2. borrow_shared_data, and 3. inner_scope_borrow
        assert_eq!(3, inner_count);
        println!("inner strong count: {inner_count}");
    }
    let count = Rc::strong_count(&shared_data);
    println!("strong count: {count}");
    // reference counts back to 2, because `inner_scope_borrow` deleted after end of inner scope above.
    assert_eq!(2, count);

    // T = Rc<U> -> &T = &U
    // &T dimana T adalah Rc<U>
    reference(&borrow_shared_data);
    dbg!(&shared_data);
    dbg!(&borrow_shared_data);

    // `borrow_shared_data` type is struct Data which doesn't implement Copy trait, hence need move ownership when value taken out of Rc, while Rc is owned by many, hence it failed.
    // T = Rc<U> -> *T = U
    // &T dimana T adalah Rc<U>
    // dereference(*borrow_shared_data);

    println!(
        "before delete: shared_data --> strong count: {:?}",
        Rc::strong_count(&shared_data)
    );
    drop(borrow_shared_data);
    println!(
        "after delete: borrow_shared_data --> strong count: {:?}",
        Rc::strong_count(&shared_data)
    );
    // Rc::try_unwrap(T) only success if there's only 1 reference left in the counting.
    // there were 2 reference before: 1. `shared_data` and 2. `borrow_shared_data`,
    // we delete reference borrow_shared_data leaving only one reference and can be unwrap without runtime panic.
    dbg!(Rc::try_unwrap(shared_data));
    // dereference(Rc::try_unwrap(borrow_shared_data).unwrap());

    let s = String::from("anu");
    let borrow_string = Rc::from(&s);
    let borrow_borrow_string = Rc::clone(&borrow_string);
    println!("borrow_string count: {}", Rc::strong_count(&borrow_string));
    println!(
        "borrow_borrow_string count: {}",
        Rc::strong_count(&borrow_borrow_string)
    );
}

fn dereference(data: Data) {
    println!("inside dereference: {:?}", data)
}

fn reference(data: &Data) {
    println!("inside reference: {:?}", data)
}
