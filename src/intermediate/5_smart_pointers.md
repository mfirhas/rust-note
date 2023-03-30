# Smart Pointers #

Smart Pointer merupakan Abstract Data Type untuk memudahkan manajemen memori tanpa reference/borrowing karena smart pointer memiliki ownership terhadap data yang di-*point*. Hampir semua smart pointer digunakan untuk manajemen data di heap dan memudahkan passing antar data tanpa harus berurusan dengan reference dan lifetime.

Kita telah menemui beberapa smart pointer sebelumnya seperti `String`, `Vec<T>`, `HashMap<K,V>`, dll. Dari contoh yang pernah kita gunakan, bisa ditarik garis besar bahwa smart pointer adalah salah satu jenis memory management high-level untuk data-data yang di store di heap karena data tersebut dynamically sized dan tidak diketahui pasti size nya saat compile time, dan berubah2. Data-data pada heap tersebut direpresentasikan oleh *handle* yang stack-allocated dan memiliki move semantics berlaku padanya, sehingga akan move atau drop jika berpindah scope. 

Smart pointer mengimplementasikan 2 trait utama di dalam rust yaitu: `Deref` dan `Drop`. `Deref` digunakan untuk me-*dereference* data yang ada di heap dan `Drop` untuk menghapus data pada heap jika sudah keluar scope. Move semantics sudah otomatis di-implement karena kebanyakan smart pointers ini dibuat menggunakan custom types seperti `struct` yang tidak meng-implement `Copy` trait sehingga move semantics berlaku padanya.

Beberapa contoh penggunaan smart pointer untuk data yang dynamic dan tidak diketahui size nya pada saat compile-time:
- User inputs.
- Recursive data.
- OOP style polymorphism (bukan ADT style polymorphism).
- Pointer pengganti reference type dan tidak ingin berurusan dengan lifetime annotations.

Kali ini kita akan membahas beberapa contoh smart pointer pada Rust:

## `Box<T>` ##
Smart pointer paling sederhana untuk mengalokasi data ke heap. Digunakan ketika kita ingin menggunakan data yang tidak diketahui size nya saat compile time(e.g. user input), dan bersifat dynamic in size.

Diantara sifat-sifat Box:
- Single ownership
- Move or drop apply if out of scope
- Immutable and mutable borrows checked at compile time

Deref Coercions:
- Dereference: `T = Box<U> -> *T = U`
- Reference:   `T = Box<U> -> &T = &U`

Contoh:
```rust
fn main() {
    do_box();
}

pub fn do_box() {
    // 123 akan di-alokasikan ke heap memory.
    // ownership rule berlaku untuk 123 di heap dan akan di-release jika keluar dari scope.
    let boxed_i32 = Box::new(123);

    // T = Box<U> -> *T = U
    // *T dimana T adalah smart pointer, akan meng-dereferensikan isi dari smart pointer
    dereference(*boxed_i32);

    // T = Box<U> -> &T = &U
    // &T dimana T adalah smart pointer, akan meng-referensikan isi dari smart pointer
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
    // T = Box<U> -> &mut T = &U
    // &mut T dimana T adalah smart pointer, akan meng-dereferensikan isi dari smart pointer
    reference_mut(&mut boxed_i32);
    println!("after reference_mut: {boxed_i32}");
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
```

## `Rc<T>` ##
Merupakan smart poninter untuk membagikan ownership terhadap suatu data(*shared ownership*) atau meminjamkan suatu data ke-berbagai borrower(*multiple borrower*) secara *immutable*. Digunakan ketika kita ingin membagian reference tanpa harus berurusan dengan lifetime dan data dapat dengan mudah di-bagikan.

Diantara sifat-sifat dari Rc:
- Multiple ownership.
- Immutable shared references(read only).
- If out of scope, heap data will be deallocated iff all references already out of scope.
- Immutable borrows checked at compile time.
- Unsafe for multithreading.

Deref Coercions:
- Reference: `T = Rc<U> -> &T = &U`
- Dereference: `T = Rc<U> -> *T = U` --> When dereferencing value inside Rc, if the type of value doesn't implement Copy trait, then ownership move happen, this will fail because Rc is shared among multiple owner, hence 1 cannot take sole ownership of it. Beside, Rc is immutable reference smart pointer, no reason to take ownership of value out of it. 

Contoh:
```rust
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
        println!("inner_scope_borrow drop here");
    }

    // T = Rc<U> -> &T = &U
    // &T dimana T adalah Rc<U>
    reference(&borrow_shared_data);
    dbg!(&shared_data);
    dbg!(&borrow_shared_data);

    // cannot move value out of Rc, because Rc is shared reference and might already shared among others, so anyone cannot take sole ownership of it.
    // T = Rc<U> -> *T = U --> FAILED, won't work for Rc smart pointer because it's shared reference sp. Make sure only 1 reference left, and use `try_unwrap()`.
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
```

## `RefCell<T>` ##
Jika `Rc<T>` bersifat immutable reference, maka kita bisa menggunakan `RefCell<T>` untuk interior mutability. Interior mutability memberikan mutability pada inner data di dalam smart pointer. Smart pointer ini memberikan fleksibilitas untuk data yang immutable, tetapi memiliki kemungkinan untuk menjadi mutable. 
Method `borrow()` return `Ref<T>` for many immutable references.
Method `borrow_mut()` return `RefMut<T>` for 1 mutable reference.

Sifat-sifat `RefCell<T>`:
- Single ownership.
- interior mutability, you can mutate value inside RefCell even if RefCell declared as immutable.
- Immutable or mutable borrows checked at runtime.
- Can panic at runtime because borrow checking happen both at runtime.
- Unsafe for multithreading.
- Doesn't implement Deref trait.

Panic cases:
- More than 1 mutable borrow.
- immutable and mutable borrow in the same scope.
- Use `try_borrow()` or `try_borrow_mut()` for non-panics way of handling borrows for immutable and mutable reference(respectively).

No Deref coercions since it's not implementing Deref trait. 
Instead, use methods `borrow()`/`try_borrow()` and `borrow_mut()`/`try_borrow_mut()` to borrow value inside immutably or mutably.

Contoh:
```rust
use std::cell::RefCell;

pub fn do_refcell() {
    let refcell_data = RefCell::new(123);

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

fn main() {
    do_refcell();
}

```

## Memory Leak of Cyclic References ##
Ada satu case memory yang tidak bisa dijamin oleh Rust yaitu reference cycle leaks. Ini merupakan suatu kondisi dimana suatu data *pointing* ke diri sendiri, atau nested data structure *pointing* ke satu sama lain. 

Contoh:
```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => Some(item),
            List::Nil => None,
        }
    }
}

fn main() {
    // a = 5 - Nil
    let a = Rc::new(List::Cons(5, RefCell::new(Rc::new(List::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    // b = 10 - a(5 - Nil)
    let b = Rc::new(List::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // a = 5 - b
    // b = 10 - a
    // a = 5 - b(10 - a(5 - b(10 - a(5 - b(10 - a(5 - b(10 - a(5 - b(10 - a(5 - ...))))))))))  --> keep referencing each others, causing leaks, stackoverflow!
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // a.tail() causes leak because above reason.
    // println!("a next item = {:?}", a.tail()); 
}
```

## `Weak<T>` for Preventing Reference Cycle ##
Setiap references menggunakan Rc menambahkan count yang bersifat strong, dalam artian selama masih ada Rc maka tidak akan direlease semua dari memory karena cycle reference menyebabkan strong count tidak sampai 0. Untuk mengatasi ini, cycle reference bisa diganti dengan `Weak<T>` yang menyebabkan semua references menggunakan `Weak<T>` akan di-release setelah semua strong count mencapai 0.

Perbedaan antara Rc strong dan weak adalah:
- strong Rc butuh count 0 untuk di-clean.
- weak Rc tidak butuh count 0 untuk di-clean, ketika **semua** strong count sudah 0, maka semua weaks akan di-clean juga.
- Rc memiliki ownership terhadap data.
- Weak tidak memiliki ownership terhadap data.

Contoh leaks scenario:
```rust
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

    arif.childs.as_ref().unwrap().borrow_mut().push(Rc::clone(&anak_arif));

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
```

Contoh non leak:
```rust
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

    arif.childs.as_ref().unwrap().borrow_mut().push(Rc::clone(&anak_arif));

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
```

Method `downgrade(&T)` digunakan untuk membuat weak Rc.
Method `upgrade()` digunakan untuk mendapat strong Rc, untuk men-dereferensikan weak reference, harus di-upgrade dulu.


## `Cow<'a, B>` ##
Smart pointer ketika kita memiliki dua kemungkinan antara *borrow* atau *own* suatu data(*optional ownership*). Kita biasa nya melakukan immutable reference terhadap data yang bersifat readonly, akan tetapi sepanjang jalan program, ada kemungkinan kita butuh mengubah data tersebut, sehingga butuh ownership untuk melakukan perubahan, bisa saja menggunakan mutable reference, tapi akan dilimit oleh beberapa limitasi mutable reference.

Cow akan memeriksa apakah `T` yang dimasukkan implement `ToOwned`.
Jika `T` merupakan owned type, maka versi reference nya akan dicek apakah implement `ToOwned` ke bentuk `T`,
Jika `T` merupakan borrowed type, maka dicek apakah implements `ToOwned`.

Cow implements `Deref` trait.

Contoh:
```rust
use std::borrow::Cow;

pub fn do_cow() {
    let literal = "anu";
    let mut new_cow = Cow::from(literal);
    let reference_new_cow = &new_cow;
    println!("{:?}", reference_new_cow);
    let dereference_new_cow = &*new_cow;
    println!("{:?}", dereference_new_cow);

    // if values already in owned form, no cloning happen. Otherwise convert it to owned form by cloning the data into heap.
    // cloning happen because slice_cow is in borrowed form &str.
    let mut_string = new_cow.to_mut();
    let owned = new_cow.into_owned();

    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let mut slice_cow = Cow::from(slice);

    println!("{:?}", slice_cow);
    let reference_slice_cow: &[i32] = &slice_cow;
    println!("{:?}", reference_slice_cow);
    let dereference_slice_cow = &*slice_cow;
    println!("{:?}", dereference_slice_cow);

    // if values already in owned form, no cloning happen. Otherwise convert it to owned form by cloning the data into heap.
    // cloning happen because slice_cow is in borrowed form &[i32].
    let mut_string = slice_cow.to_mut();
    let owned = slice_cow.into_owned();

    let v = vec![1, 2, 3, 4, 5];
    let mut cow_v = Cow::from(v);

    match cow_v {
        Cow::Borrowed(x) => println!("{:?} is borrowed", x),
        Cow::Owned(ref x) => println!("{:?} is owned", x),
    }

    // no clone happen because v already in owned form.
    let mut_vec = cow_v.to_mut();
    let owned_vec = cow_v.into_owned();

    let command = "this should be all upperCase";
    println!("{:?}", check_uppercase(command));

    let v = vec![1, -2, 3, -4, -5];
    let input = Cow::from(v);
    // no cloning happen, because v already in owned form.
    println!("{:?}", check_vector(input));

    let v = &[1, -2, 3, -4, -5] as &[i32];
    let input = Cow::from(v);
    // cloning happen because v is in borrowed form &[i32].
    println!("{:?}", check_vector(input));
}

// this function return either borrowed &str or owned String
fn check_uppercase(s: &str) -> Cow<str> {
    if s.chars().all(|x| x.is_uppercase()) {
        return s.into();
    }

    s.to_uppercase().into()
}

fn check_vector(v: Cow<[i32]>) -> Vec<i32> {
    // if v already in owned form, no cloning happen, otherwise clone it and take ownership
    let it = v.into_owned().into_iter(); // if v is owned form, this line is zero cost, otherwise costing it cloning into heap.
    let ret = it.filter(|x| x.is_positive()).collect::<Vec<i32>>();
    ret
}
```

## `Arc<T>` ##
Sama seperti `Rc<T>`, bedanya kalau `Arc<T>` bersifat thread-safe dalam artian bisa digunakan antar threads.
Rc merupakan shared ownership, dalam artian setiap owner bisa melakukan perubahan terhadap data yang ada di dalamnya, hal ini tidak masalah jika komputasi berada dalam ranah sinkronus, akan tetapi menjadi masalah jika dalam ranah asinkronus karena akan menyebabkan data race dan UB.
Untuk memberikan sifat atomic pada data yang di-share antar threads, maka dibutuhkan `Arc`(Atomic Reference Counting).

Contoh:
```rust
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;
use std::thread::spawn;

pub fn do_arc() {
    let arc_data = Arc::new(123);

    // deref coercions:
    let ref_arc_data: &i32 = &arc_data;
    let deref_arc_data = *arc_data;

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
```

## `Mutex<T>` ## 
Sama seperti kombinasi `Rc<T>` dan `RefCell<T>` untuk shared multiple ownership with interior mutability, maka untuk `Arc<T>` ada `Mutex<T>` untuk interior mutability yang thread-safe antar threads. Mutex merupakan smart pointer untuk melindungi shared mutable data antar threads dari data race dan UB.

Contoh:
```rust
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
```
Kita menggunakan contoh pada kasus Rc dan RefCell di atas, akan tetapi kali ini untuk thread-safety karena kita akan mem-passing data antar threads. *Data race free* dan *no leaking*.