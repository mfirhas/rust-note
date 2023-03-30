use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Weak;
use std::thread;

#[derive(Debug)]
pub struct Person {
    name: String,
    shared_wealth: Arc<Mutex<i32>>,
    // no cyclic reference happen here because bind to parent is weak so child doesn't own its parent, otherwise parent own its child.
    parent: Mutex<Weak<Person>>,
    childs: Option<Mutex<Vec<Arc<Person>>>>,
}

pub fn do_mutex() {
    let shared_wealth = Arc::new(Mutex::new(0));

    let arif = Arc::new(Person {
        name: "arif".to_string(),
        shared_wealth: Arc::clone(&shared_wealth),
        parent: Mutex::new(Weak::new()),
        childs: Some(Mutex::new(vec![])),
    });

    let anak_arif = Arc::new(Person {
        name: "ahmad".to_string(),
        shared_wealth: Arc::clone(&shared_wealth),
        parent: Mutex::new(Arc::downgrade(&arif)),
        childs: Some(Mutex::new(vec![])),
    });

    arif.childs
        .as_ref()
        .unwrap()
        .lock()
        .unwrap()
        .push(Arc::clone(&anak_arif));

    dbg!(&arif); // read arif
    dbg!(&arif.childs.as_ref().unwrap().borrow()); // read arif's child
    dbg!(&arif
        .childs
        .as_ref()
        .unwrap()
        .lock()
        .unwrap()
        .get(0)
        .unwrap()
        .parent
        .lock()
        .unwrap()
        .upgrade()); // read arif's child's parent, which is arif himself
    dbg!(&arif.parent); // read arif's parent

    let parent_working = Arc::clone(&arif);
    let parent_work = thread::spawn(move || {
        for _ in (0..10) {
            *parent_working.shared_wealth.lock().unwrap() += 1;
        }
    });

    let child_working = Arc::clone(&anak_arif);
    let child_work = thread::spawn(move || {
        for _ in (0..10) {
            *child_working.shared_wealth.lock().unwrap() += 1;
        }
    });

    parent_work.join().unwrap();
    child_work.join().unwrap();
    dbg!(&arif);
    let total_wealth = *arif.shared_wealth.lock().unwrap();
    assert_eq!(20, total_wealth);
}
