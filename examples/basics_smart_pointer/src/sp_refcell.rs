use std::{cell::RefCell, ops::Add};

pub fn do_refcell() {
    let refcell_data = RefCell::new(123);
    let reference = &refcell_data;
    // let reference: &i32 = &refcell_data; // RefCell doesn't implement Deref trait
    // let dereference = *refcell_data; // RefCell doesn't implement Deref trait
    // let f = refcell_data.try_borrow();
    // let g = refcell_data.try_borrow_mut();

    // ! Doing immutable and mutable reference in one scope will panic as value already borrow while mutating in the same scope.
    // let borrow = refcell_data.borrow();
    // let borrow_mut = *refcell_data.borrow_mut() + 123;

    *refcell_data.borrow_mut() = 456;
    dbg!(refcell_data.borrow());
    *refcell_data.borrow_mut() += 123;
    dbg!(refcell_data.borrow());

    let mut borrow_mut = refcell_data.borrow_mut();
    dbg!(&borrow_mut);
    *borrow_mut = 900;
    dbg!(&borrow_mut);
    *borrow_mut += 4;
    dbg!(&borrow_mut);
}
