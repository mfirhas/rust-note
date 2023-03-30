use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct Person {
    name: String,
    // no cyclic reference happen here because bind to parent is weak so child doesn't own its parent, otherwise parent own its child.
    parent: RefCell<Weak<Person>>,
    childs: Option<RefCell<Vec<Rc<Person>>>>,
}

pub fn do_weak() {
    let arif = Rc::new(Person {
        name: "arif".to_string(),
        parent: RefCell::new(Weak::new()),
        childs: Some(RefCell::new(vec![])),
    });

    let anak_arif = Rc::new(Person {
        name: "ahmad".to_string(),
        parent: RefCell::new(Rc::downgrade(&arif)),
        childs: Some(RefCell::new(vec![])),
    });

    arif.childs.as_ref().unwrap().borrow_mut().push(anak_arif);

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
        .borrow()
        .upgrade()); // read arif's child's parent, which is arif himself
    dbg!(&arif.parent.borrow().upgrade()); // read arif's parent
}
