# Subtyping #

Subtyping adalah teknik memberikan sifat ***substitutability*** terhadap tipe data karena adanya relasi tertentu antar tipe data tersebut. Teknik ini merupakan upaya untuk memberikan semacam *polymorphism* terhadap tidak data sehingga lebih flexible dan *safe* dalam penggunaannya. Teknik ini berkembang menjadi salah satu prinsip dalam pemograman yang dikenal dengan Liskov Substitution Principle.

Setiap bahasa memiliki *notion* yang berbeda terhadap subtyping, bahkan ada yang tidak memiliki subtyping sekalipun.

Pada Rust, ada beberapa subtyping diantaranya:
- Closures and Function pointer
- Immutability and Mutability
- Lifetime

## Closures and Function Pointer ##
Closures dan Function Pointer saling substitutable satu sama lain.

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

fn return_function_pointer() -> impl FnMut(i32) -> i32 {
    fp
}

fn return_closure_from_inside() -> fn(i32) -> i32 {
    |x| x
}

fn fp(a: i32) -> i32 {
    a
}

pub fn f_a() {
    let c = |x: i32| -> i32 { x };
    let a = A { f: c, g: f_lt };
    println!("{}", (a.f)(123));
    println!("{}", (a.g)(&234));

    accept_closure(f, 5);
    return_closure()(5);
    println!("123-> {}", return_function_pointer()(123));
    println!("234-> {}", return_closure_from_inside()(234));
}
```

## Immutability and Mutability ##
Value yang bersifat immutable dan mutable memiliki relasi subtyping tersendiri, yaitu: 
- Mutable value invariant terhadap sesama mutable value, 
- Mutable value covariance terhadap immutable value, meaning mutable is subtype of immutable (Mu <: Immu).

```rust
// mutable invariances
let a: &str = "this";
accept_immutable(a);
// accept_mutable(a); // failed because parameter is mutable and only accept mutable since it's invariant

// mutable covariance over immutable
let mut a: &str = "this";
accept_immutable(a); // works because parameter is immutable and mutable values are covariance over immutable parameters.
accept_mutable(&mut a);

fn accept_immutable(a: &str) {
    // even if a get value from mutable reference, it becomes immutable in this function scope.
}
fn accept_mutable(a: &mut &str) {
    *a = "lskdfm";
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

        // compiler is smart enough to shorten d's lifetime from 'b(longer) to 'a(shorter) since it's only used here and getting return value of 'a lifetime from function `lifetime_a`
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
