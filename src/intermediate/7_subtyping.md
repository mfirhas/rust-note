# Subtyping #

Subtyping adalah cara Rust untuk memberikan relasi terhadap tipe data. Subtyping dilakukan untuk memberikan fleksibilitas terhadap penggunaan tipe data. Contoh subtyping sudah kita bahas sebelumnya salah satunya dalam OOP yaitu inheritance, dimana relasi yang terjadi ada parent-child. Definisi lain menyatakan inheritance bukan subtyping, akan tetapi interface lah yang merupakan subtyping, karena memberikan sifat *substitutability* pada tipe data/objek.

Pada Rust, ada beberapa subtyping diantaranya:
- Trait
- Closures and Function pointer
- Lifetime

## Trait ##
Trait bisa di-ekstensi menggunakan notasi `:`, menjadi subtrait. Semua type yang implements suatu subtrait, juga harus implement supertrait nya, tapi tidak sebaliknya.

Contoh:
```rust
/// subtyping in rust, replacing inheritance.

// trait subtyping
pub trait Base {
    fn walk(&self);
}

pub trait Child: Base {
    fn run(&self);
    fn fly(&self);
}

#[derive(Debug, Clone)]
pub struct A {
    name: String,
}

impl A {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Base for A {
    fn walk(&self) {}
}

impl Child for A {
    fn run(&self) {}
    fn fly(&self) {}
}

#[derive(Debug, Clone)]
pub struct B {
    name: String,
}
impl B {
    pub fn new(name: &str) -> Self {
        B {
            name: String::from(name),
        }
    }
}
impl Base for B {
    fn walk(&self) {}
}

fn process<T: Base>(b: T) {
    println!("process called");
    b.walk();
}

fn process_return_base() -> impl Base {
    B::new("anu")
}

fn process_sub<C: Child>(c: C) {
    println!("process_sub called");
    c.run();
    c.fly();
}

pub fn do_trait() {
    let a = A {
        name: "this A".to_string(),
    };
    process(a.clone());
    process_sub(a);

    let b = B {
        name: "this B".to_string(),
    };
    process(b.clone());
    // process_sub(b); // failed, because process_sub require Child trait, subtype of Base trait, but B only implement Base trait.
    let base = process_return_base();
}
```

## Closures and Function Pointer ##
Closures merupakan supertype dari function pointer, sehingga bisa saling substituable/covariant.

```rust
struct A {
    f: fn(i32) -> i32,
    g: for<'a> fn(&'a i32) -> &'a i32,
}

fn accept_closure<F: Fn(i32) -> i32>(f: F, x: i32) {
    println!("{}", f(x))
}

fn return_closure() -> impl Fn(i32) -> i32 {
    f
}

fn f(x: i32) -> i32 {
    x
}

fn f_lt(x: &i32) -> &i32 {
    x
}

pub fn f_a() {
    let c = |x: i32| -> i32 { x };
    let a = A { f: c, g: f_lt };
    println!("{}", (a.f)(123));
    println!("{}", (a.g)(&234));

    accept_closure(f, 5);
    return_closure()(5);
}
```

## Lifetime ##
Lifetime bisa memiliki subtyping, disebut outliving. Ketika 'a: 'b, berarti lifetime a outlives lifetime b.

```rust
// lifetime subtyping
pub fn lifetime_subtyping() {
    // 'b
    let b = String::from("lifetime b"); // lifetime of b is 'b
    let c;
    let d;
    {
        // 'a
        let a = String::from("lifetime a"); // lifetime of a is 'a
        c = lifetime_b(&a, &b);
        /// error, because c takes lifetime of a which is the current lifetime, while c has type &'b str, and 'a is not subtype of 'b, instead 'b is subtype of 'a
        /// because b live longer than a, b outlives a.
        // c = lifetime_a(&a, &b);
        println!("{c}");

        // d get value of lifetime 'a, which is the current lifetime.
        // while lifetime of d declared above is 'b.
        // this works because lifetime is covariant each other.
        // means that lifetime can be substituted with its variances.
        // d is declared inside lifetime 'b, and 'b is subtype of 'a, hence 'a and 'b have relations eachother that 'b extends 'a lifetime.
        // based on this, d will have shorter lifetime than it was declared above, hence cannot live beyond this scope.
        d = lifetime_a(&a, &b);
        println!("{d}");
    }
    /// can still access c, because c lifetime is 'b, and value of c returned from lifetime_b() takes lifetime of b, and b is beyond a.
    println!("{c}");

    // ! will error because d lifetime has been shortened to 'a inside above scope.
    // println!("{d}");
}

// 'b: 'a means b lifetime outlives a, means b live longer than a, means b should live at least as a, means a must die first then b.
fn lifetime_b<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'b str {
    b
}

fn lifetime_a<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
    a
}
```

# Variances #
Subtyping lebih lanjut membahas tentang type compatibility atau *variances*. Terdapat 3 jenis variances dalam subtyping:
- Covariant     = Ketika **subtype** bisa di-pass ke dalam **supertype**, T <: S then T -> S, S(supertype) substituable oleh T(subtype). ***More Specific***.
- Contravariant = Ketika **supertype** bisa di-pass ke dalam **subtype**, T <: S then S -> T, T(subtype) substituable oleh S(supertype). ***More Generic***.
- Invariant     = Ketika tidak ada hubungan subtyping antara dua tipe. Dalam artian tipe yang di-pass haruslah yang sama persis.

Berikut beberapa contoh variances:
- `&T` & `const T`: covariant over T. T is subtituable.
- `&mut T` & `*mut T`: invariant over T. T is not substituable, must exact same thing.
- `&'a T`: covariant terhadap lifetime 'a dan T. 
  - Substituability applied to lifetime 'a.
  - Kita bisa mem-passing mutable T ke immutable T.
- `&'a mut T`: covariant terhadap lifetime 'a, dan invariant terhadap T.
  - Substituability applied to lifetime 'a.
  - Kita hanya bisa mem-passing exact mutable T ke mutable T, Sekalipun T punya relasi subtype/supertype.
- Smart pointer with **no** interior mutability(Box, Rc, Arc, etc), covariant over its contained types.
  - `Box<T>` is covariant over T. `T` <: `S` -> `Box<T>` <: `Box<S>`.
- Smart pointer with interior mutability(`Cell<T>`, `RefCell<T>`, `UnsafeCell<T>`, etc), invariant over its contained types.
  - `RefCell<T>` is invariant over T, artinya T tidak bisa di-substitute oleh hal lain, alias harus sama persis.
- `fn(T) -> U`: contravariant over T, covariant over U.

Contoh:
```rust
// source: https://stackoverflow.com/questions/55344893/what-is-an-example-of-contravariant-use-in-rust

// contravariances
struct MyContraType<Mixed> {
    k1: fn(Mixed), // contravariant over Mixed
}

// Rust is contravariant over function parameters
fn contra_example<'short>(
    mut a: MyContraType<&'short u8>,
    mut b: MyContraType<&'static u8>,
    x: fn(&'short u8),
    y: fn(&'static u8),
) {
    // a = MyContraType<short>.fn(short)
    // b = MyContraType<static>.fn(static)
    // x = fn(short)
    // y = fn(static)
    // static: short
    // short is supertype
    // static is subtype
    // static <: short

    // x(short)  passed into a.k1(short)  = bivariant,     passed
    a.k1 = x;

    // y(static) passed into a.k1(short)  = covariant,     failed
    // because function parameter is contravariant, so we cannot pass subtype(static) into supertype(short)
    // a.k1 = y; // Fails

    // x(short)  passed into b.k1(static) = contravariant, passed
    // static <: short ==> x(short) <: b.k1(static)
    // subtype(static) substituable by supertype(short)
    b.k1 = x;

    // y(static) passed into b.k1(static) = bivariant,     passed
    b.k1 = y;
}

// covariances
struct MyCoType<Mixed> {
    k1: fn() -> Mixed, // covariant over Mixed
}

fn co_example<'short>(
    mut a: MyCoType<&'short u8>,
    mut b: MyCoType<&'static u8>,
    x: fn() -> &'short u8,
    y: fn() -> &'static u8,
) {
    // a = MyCoType<short>.fn() -> short
    // b = MyCoType<static>.fn() -> static
    // x = fn() -> short
    // y = fn() -> static
    // static: short
    // short is supertype
    // static is subtype
    // static <: short

    // x() -> short passed into a.k1() -> short = bivariant, passed
    a.k1 = x;

    // y() -> static passed into a.k1() -> short = covariant, passed
    // static <: short ==> y() -> static <: a.k1() -> short
    // supertype(short) substituable by subtype(static)
    a.k1 = y;

    // x() -> short passed into b.k1() -> static = contravariant, failed
    // because function return is covariant, so we cannot pass supertype(short) into subtype(static)
    // b.k1 = x; // Fails

    // y() -> static passed into b.k1 -> static = bivariant, success
    b.k1 = y;
}
```
