/// ! THIS CAUSE MEMORY LEAKS AS THERE'S CYCLIC REFERENCE IN THE CODE.
///
///
use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Person {
    name: String,
    // cyclic reference happen between parent and childs because parent's child's parent's child's parent's ...
    parent: Option<RefCell<Rc<Person>>>,
    childs: Option<RefCell<Vec<Rc<Person>>>>,
}

pub fn do_rc_leak() {
    let arif = Rc::new(Person {
        name: "arif".to_string(),
        parent: None,
        childs: Some(RefCell::new(vec![])),
    });

    let anak_arif = Rc::new(Person {
        name: "ahmad".to_string(),
        parent: Some(RefCell::new(Rc::clone(&arif))),
        childs: Some(RefCell::new(vec![])),
    });

    arif.childs
        .as_ref()
        .unwrap()
        .borrow_mut()
        .push(Rc::clone(&anak_arif));

    dbg!(&arif); // read arif
    dbg!(&arif.childs.as_ref().unwrap().borrow()); // read arif's child
    dbg!(&arif
        .childs
        .as_ref()
        .unwrap()
        .borrow()
        .get(0)
        .unwrap()
        .parent
        .borrow()); // read arif's child's parent, which is arif himself
}
