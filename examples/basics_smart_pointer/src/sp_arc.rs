use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::spawn;

pub fn do_arc() {
    let arc_data = Arc::new(123);

    // deref coercions:
    let ref_arc_data: &i32 = &arc_data;
    println!("{:?}", ref_arc_data);
    let deref_arc_data = *arc_data;
    println!("{:?}", deref_arc_data);

    let clone_arc_data = Arc::clone(&arc_data);
    let ref_arc_data: &i32 = &clone_arc_data;
    println!("{:?}", ref_arc_data);
    let deref_arc_data = *clone_arc_data;
    println!("{:?}", deref_arc_data);

    reference(&clone_arc_data);
    dereference(*clone_arc_data);

    let da = Rc::new(123);
    let b = *da;
    let c = &da;

    // cannot send Rc between threads, because Rc doesn't implement Send trait
    // let data = Rc::new(RefCell::new(123));
    // let thread_1 = spawn(move || {
    //     *data.borrow_mut() += 1;
    // });
    // thread_1.join().unwrap();
    // println!("{:?}", data);

    // cannot send Rc between threads, because Rc doesn't implement Send trait
    // let data = Rc::new(123);
    // let thread_1 = spawn(move || {
    //     println!("{:?}", data);
    // });
    // thread_1.join().unwrap();
    // println!("{:?}", data);

    let data = Arc::new(123);
    let thread_1 = spawn(move || {
        println!("{:?}", data);
    });
    thread_1.join().unwrap();
}

fn reference(a: &i32) {
    println!("reference: {}", a);
}

fn dereference(a: i32) {
    println!("dereference: {}", a);
}
