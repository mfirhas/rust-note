# Trait #

Trait merupakan tipe data *opaque* yang tidak memiliki tipe data konkrit, akan tetapi hanya mengandung behaviour(collection of methods/functions). Tipe ini bisa digunakan untuk mengimplementasikan *polymorphism* dimana semua tipe bisa diterima selama memiliki behaviour yang sama.
Semua method-method yang ada pada trait adalah public, sehingga ketika meng-implementasikan trait tidak perlu mendeklarasikan method sebagai public.

Trait dapat digunakan untuk beberapa hal di-antaranya:
- *Shared behaviour*: banyak tipe bisa mengimplementasikan behaviour yang sama(*polymorphism*). Kegunaan lain adalah untuk membuat objek *mock* yang bisa digunakan untuk unit testing, khususnya IO bound operations.
- *Generic bound*: bisa digunakan untuk memberikan batasan tipe generic, sehingga memperkecil tipe data yang bisa dimasukkan ke dalam generic dan memberikan API yang lebih konkrit dan mudah dipahami dan digunakan.

Trait dapat digunakan di tempat-tempat berikut(V1.64+):
- Function input and return params
- Type aliasing
- Bound for generic type

Trait dapat diimplementasikan oleh:
- struct
- enum
- primitive types
- non-primitive types

```rust
pub trait MyTrait {
    fn method1(&self) -> i32;
    fn method2(param1: &str) -> Result<(),String>;
    fn method3(param1: String);
}
```

Implementasi trait harus eksplisit sehingga harus di-define untuk masing-masing type yang ingin mengimplementasikan trait tersebut.
Berikut contoh implementasi trait dan beberapa cara memanggilnya:
```rust
// Struct yang akan mengimplementasikan trait `MyTrait`
pub struct MyStruct {
    a: i32,
    b: String,
}

// Implementasi MyTrait oleh MyStruct
impl MyTrait for MyStruct {
    // implementasi trait dengan immutable reference dari struct implementor.
    // struct harus punya nilai terlebih dahulu sebelum memanggil method ini dan diikuti `.`
    fn method1(&self) -> i32 {
        self.a
    }

    // implementasi trait tanpa menginisialisasi struct implementor.
    // cukup dengan memanggil type struct diikuti `::`
    fn method2(param1: &str) -> Result<(),String> {
        if param1.is_empty() {
            return Err("empty string".to_string());
        }
        Ok(())
    }

    fn method3(param1: String) {
        println!("do nothing");
    }
}
```

Beberapa cara pengaplikasian trait:
```rust
// Param input fungsi
let ms = MyStruct {
    a: 1,
    b: String::from("test"),
};

fn accept_trait(t: &impl MyTrait) {
    let resp = t.method1();
    dbg!("accept_trait: ",resp);
    let ret = MyStruct::method2("param1");
    dbg!("accept_trait: ",ret);

    MyStruct::method3("nothing inside accept_trait: ".to_string());
}

accept_trait(&ms);

// Param output fungsi
fn return_trait() -> impl MyTrait {
    MyStruct {
        a: 100,
        b: "test".to_string(),
    }
}
let t = return_trait();

// Param input dan output fungsi from associated function
struct F {
    a: i32,
}
impl F {
    fn accept_trait_from_impl(param: &impl MyTrait) {
        param.method1();
    }

    fn return_trait_from_impl() -> impl MyTrait {
        MyStruct {
            a: 123,
            b: String::from("sdf"),
        }
    }
}

// membatasi method2 yang akan dipanggil dengan menggunakan `as Trait`
// disebut juga dengan Fully Qualified Syntax
let ret = <MyStruct as MyTrait>::method2("param1");
```

Ketika ingin memanggil fungsi/method trait dari module lain yang diimplementasikan oleh suatu implementor, tipe trait tersebut harus dibawa ke dalam scope caller. Contoh:

file: src/a.rs
```rust
pub trait T {
    fn t(&self);
    fn t2();
}

pub struct S {
    pub s: String,
}

impl T for S {
    fn t(&self) {
        println!("{}", self.s);
    }
    fn t2() {}
}
```

file: src/main.rs
```rust
mod a;
use a::{S, T};

fn main() {
    let sss = S {
        s: "sdf".to_string(),
    };
    sss.t();
    S::t2();
}
```

Trait `T` harus dideklarasikan lewat module `a` sekalipun trait tersebut tidak pernah digunakan secara langsung, akan tetapi ada tipe lain yang implement trait tersebut, memanggil fungsi/method dari trait tersebut. Jika trait tidak dibawa ke dalam scope, maka akan menyebabkan kompilasi error karena compiler tidak bisa menemukan fungsi/method yang dipanggil oleh struct `S`, karena fungsi/method tersebut didefinisikan oleh trait bersangkutan.

Selain `struct`, trait juga bisa diimplementasikan oleh tipe lain seperti `enum`, primitive types, dan non-primitive types. Contoh:

file: src/a.rs
```rust
pub trait Trait {
    fn t(&self) -> i32;
    fn t2();
}

pub enum Enum {
    F,
    G,
    H(String),
}

impl Trait for Enum {
    fn t(&self) -> i32 {
        match &*self {
            Enum::H(s) => {
                println!("{}", s);
            }
            _ => {
                println!("nothing");
            }
        }
        123
    }
    fn t2() {
        
    }
}

impl Trait for i32 {
    fn t(&self) -> i32 {
        *self
    }
    fn t2() {
        println!("i32::t2");
    }
}

impl Trait for &[i32] {
    fn t(&self) -> i32 {
        *self.get(0).unwrap_or(&123)
    }
    fn t2() {
        println!("&[i32]::t2");
    }
}

impl Trait for [f32;5] {
    fn t(&self) -> i32 {
        if !self.is_empty() {
            return *self.get(0).unwrap() as i32;
        }
        123
    }

    fn t2() {
        println!("[f32;5]::t2");
    }
}

impl Trait for Vec<String> {
    fn t(&self) -> i32 {
        if !self.is_empty() {
            let parsed_string = self.get(0).unwrap().parse::<i32>();
            if parsed_string.is_err() {
                return 0;
            }
            return parsed_string.unwrap();
        }
        123
    }

    fn t2() {
        println!("Vec<String>::t2");
    }
}
```

Ketika ingin memanggil implementasi2 trait tersebut dari `src/a.rs`:

file: src/main.rs
```rust
let e = Enum::H("anu".to_string());
e.t();
Enum::t2();

// -----------

let num = 123;
println!("{}",num.t());
i32::t2();

// -----------
let slice: &[i32] = &[1,2,3,4];
println!("{}", slice.t());
<&[i32]>::t2();

// -----------
let array = [1.2, 2_f32, 3_f32, 4.2, 5.2];
println!("{}", array.t());
<[f32;5]>::t2();

// -----------
let v = vec![String::from("345"), "that".to_string(), "asd".to_owned()];
println!("{}",v.t());
<Vec<String>>::t2();
```

Untuk tipe data non-primitive seperti array, slice, vector dan lainnya, untuk memanggil associated functions dari tipe-tipe tersebut, maka harus menggunakan `Fully Qualified Syntax` dengan cara:
```rust
<type>::function();
```

## Default Implementation ##
Ketika mendefinisikan suatu trait, dan ingin mengimplementasikan trait tersebut oleh suatu tipe, kadang kita belum tentu ingin mengimplementasikan seluruh fungsi/methods yang ada, untuk hal ini, kita bisa define default implementation untuk sebagian atau keseluruhan fungsi. 

file: src/default.rs
```rust
pub trait D {
    fn de(&self) -> String {
        String::from("default")
    }
    fn not_de() -> String;
}

pub struct P;

impl D for P {
    fn not_de() -> String {
        String::from("P")
    }
}

pub struct O;

impl D for O {
    fn de(&self) -> String {
        String::from("overwrited")
    }
    fn not_de() -> String {
        String::from("O")
    }
}
```

Ketika ingin memanggil:

file: src/main.rs
```rust
let p = P;
println!("{}", p.de());
println!("{}", P::not_de());
let o = O;
println!("{}", o.de());
println!("{}", O::not_de());
```

Default implementation tidak mengharuskan tipe implementor mengimplementasikan fungsi/method dengan default behaviour tersebut. Akan tetapi jika kita mengimplementasikan fungsi/method dengan default behaviour, maka akan meng-overwrite default behaviour tersebut.

## Trait Associated Type ##
Merupakan placeholder type untuk tipe data yang akan digunakan oleh method-method pada trait.

Perbedaan penggunaan associated type dengan generics, adalah dengan menggunakan associated API implementor tidak perlu membuat implementasi generic untuk suatu tipe, karena hal ini terlihat seolah-olah memiliki berbagai jenis tipe. Misal jika kita ada `trait Trait<T> {...}`, maka akan bisa di deklarasi berbagai tipe untuk T seperti `Trait<String>`, `Trait<i32>`, etc. Dari sudut pandang API user, mereka tidak perlu memberi spesifikasi tipe, karena implementor sudah meng-enkapsulasi type implementor dan tipe yang terasosiai ke dalam satu trait tersebut. API user bahkan tidak perlu membawa definisi trait ke dalam scope hanya untuk memenuhi fully qualified syntax dengan spesifikasi tipe generic. User hanya cukup tau suatu tipe sudah mengimplementasi suatu trait dengan semua data yang terasosiasi. Secara idiom, generic type pada trait menggunakan associated type, dan generic type sebenarnya akan digunakan oleh implementor type.

Contoh:
```rust
use std::fmt::Debug;

pub trait TraitAss {
    type Item: Sized + Debug + Default;

    fn method1(&self) -> &Self::Item;

    fn method2(&self) -> (Self::Item, i32) {
        (Default::default(), 0)
    }
}

pub struct Struct<T> {
    field1: i32,
    field2: T,
}
impl<T> Struct<T> {
    pub fn new(name: T) -> Self {
        Self {
            field1: 123,
            field2: name,
        }
    }
}

impl<T> TraitAss for Struct<T>
where
    T: Sized + Debug + Default,
{
    type Item = T;

    fn method1(&self) -> &Self::Item {
        &self.field2
    }
}
```