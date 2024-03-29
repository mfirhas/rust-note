# Subtyping #

Subtyping merupakan relasi suatu tipe data dengan tipe data lain, sehingga suatu tipe data bisa digantikan dengan tipe data lain(Substituable).

Rust tidak memiliki konsep Subtyping seperti pada kebanyakan bahasa OOP lainnya, khususnya yang berhubungan dengan inheritance subtyping. Satu-satunya subtyping pada Rust adalah *lifetime* subtyping, yaitu ketika kita melakukan *borrowing* terhadap suatu data, maka lifetime dari *pinjaman* tersebut bisa memiliki relasi dengan lifetime lainnya atau disebut *outliving*. 

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
