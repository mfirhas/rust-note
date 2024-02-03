# Object Oriented Programming in Rust #

OOP memiliki banyak definisi seiring waktu. Bermula dari Simula yang mengenalkan konsep object, program disusun sedemikian rupa dari sekumpulan object-object yang dibangung dari class, attributes, dan virtual procedures(methods). Selanjutnya oleh Smalltalk menambahkan konsep message passing, dimana object akan menerima suatu *message* melalui method-method nya, dan menyesuaikan state berdasarkan itu. Java datang dengan konsep yang mirip-mirip dan dijadikan acuan dalam OOP yaitu memiliki:
- Encapsulation: Data dan Behaviours di-*wrap* menjadi satu dan interaksi terjadi melalui access modifier terhadap public methods mutating shared data.
- Abstraction: Memodelkan komponen yang memiliki kesamaan dalam attributes/data dan behaviours/methods dan menyembunyikan implementation details.
- Inheritance: Memberikan relasi antar komponen code dengan relasi "is a". Digunakan untuk code reuse dan ekstensi attributes dan behaviours.
- Polymorphism: Deklarasi common behaviour antar multiple objects. Diimplementasikan dengan menggunakan Vtable, dynamically checked.

## Encapsulation ##
Pada Rust enkapsulasi bisa dicapai melalui associated items, yaitu ketika User-Defined Types(Struct & Enum) memiliki fields/attributes dan `impl`.

Contoh:
```rust
pub Object struct {
    field1: String,
    field2: i32,
}

impl Object {
    pub fn method1(&self) -> String {
        // ...
    }

    pub fn method2(&mut self) -> String {
        // ...
    }

    pub fn method3(self) -> String {
        // ...
    }
}
```

Perbedaan enkapsulasi antara bahasa OOP umumnya(Java, C#, etc) dengan Rust adalah, pada kebanyakan bahasa OOP lainnya, object memiliki dynamic dispatch dan juga kadang late binding(runtime binding). Hal ini biasanya diimplementasikan melalui semacam "informasi" yang diembed pada runtime dan diakses pada saat program berjalan, biasanya disebut *vtable*. Pada Rust, associated items bersifat static dan compile-time binded(di tambah dengan sistem ownership dan borrow).

## Abstraction ##
Bisa dicapai dengan memanfaatkan:

- Module system pada rust, dengan menciptakan inner module for implementation details untuk public API/interfaces
Pada Rust bisa memanfaatkan access modifier seperti `pub`(public interfaces), `pub(self)`(private), `pub(crate)`(same crate only), `pub(super)`(protected).

- Trait, dengan associated types untuk common attributes shared, dan default implementations untuk common behaviours shared, serta required methods untuk common behaviours with different implementations. Contoh: trait [Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html).
Iterator adalah contoh abstraksi untuk tipe data collections seperti list, map, tree, dan lainnya.

## Inheritance ##
Rust tidak memiliki inherintance layaknya kebanyakan bahasa OOP lainnya. Hal ini serupa dengan kebanyakan bahasa FP lainnya. Hal ini karena sifat strongly typed setiap tipe data pada Rust, dan juga implementasi Algebraic Data Types. Inherintance pada Rust bisa diganti dengan komposisi, atau memanfaatkan `trait`.

## Polymorphism ##
Bahasa seperti Java, memiliki beberapa jenis polymorfisme seperti methods overloading, methods overriding, coercions, subtyping dan parametric.

Di antara polimorfisme pada Rust:
- Operator overloading
- Coercions: konversi tipe secara implisit seperti `let a: f32 = 123_i32 as f32;`, `&mut T -> &T`, dll. (more)[https://doc.rust-lang.org/reference/type-coercions.html#coercion-types]
- Lifetime polymorphism melalui lifetime behavioural subtyping(Liskov Substitution Principle)
- `std::any` & function pointer, *dynamically typed* and procedural polymorphism. (Procedural Programming style of Polymorphism)
- Trait object, dynamically dispatched. (Object-Oriented Programming style polymorphism)
- Parametric Polymorphism menggunakan traits dan type parameter. (Functional Programming style polymorphism)

Rust tidak memiliki method/function overloading, hal ini berkaitan dengan optimisasi yang berhubungan dengan penerapan generic. Instead, kita bisa menerapkan method/function overloading dengan cara generic/parametric polymorphism dengan trait dan associated types.

## Polymorphism menggunakan trait object(`dyn Trait`) ##
Polymorphism pada bahasa OOP umumnya merupakan dynamic polymorphism yang menggunakan `interface` untuk memasukkan object yang share common behaviours.
Rust menggunakan keyword `dyn` pada trait, untuk menandakan bahwa trait tersebut bisa menampung banyak tipe.

Apa yang membedakan generics dan trait object?
- generics di-resolve pada saat compile time, trait object di-resolve pada saat run-time.
- generics hanya bisa memiliki 1 jenis tipe data pada satu waktu(saat definisi tipe), sedangkan trait object bisa memiliki banyak jenis tipe data pada satu waktu(saat definisi tipe).
- generics bersifat zero cost, sedangkan trait object memiliki cost run-time vtable checking.

Contoh:
```rust
use std::fmt::Debug;

// hanya bisa deklarasi i32 pada saat definisi tipe Vec<T> -> Vec<i32>
let v = vec![1,2,3,4]; 

// hanya bisa deklarasi String pada saat definisi tipe Vec<T> -> Vec<String>
let v: Vec<String> = vec!["test".to_string(), "test2".to_string()]; 


/// contoh penggunaan trait object untuk menampung berbagai tipe data pada saat definisi tipe.
pub trait Field: Debug {}
#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}

let mut v: Vec<Box<dyn Field>> = Vec::new();
v.push(Box::new(123));
v.push(Box::new("anu"));
v.push(Box::new(String::from("nmnm")));
v.push(Box::new(Custom));

println!("{v:?}");
```

## Composition ##
Kita bisa meng-elaborasi polymorphism di atas menjadi komposisi yang terdiri dari berbagai tipe object, object disini merupakan trait object.

Contoh: 
```rust
use std::fmt::Debug;

/// composition
///
///

// IDep merepresentasikan "interface" ke depedensi yang akan di-inject
pub trait IDep: Debug {}
pub trait Field: Debug {}
#[derive(Debug)]
pub struct Custom;

impl Field for i32 {}
impl Field for String {}
impl Field for &str {}
impl Field for Custom {}

#[derive(Debug)]
pub struct Struct {
    field1: Box<dyn IDep>,
    field2: Vec<Box<dyn Field>>,
}

// Dep adalah object dependensi itu sendiri
#[derive(Debug)]
pub struct Dep;
impl IDep for Dep {}

impl Struct {
    pub fn new() -> Self {
        let mut v: Vec<Box<dyn Field>> = Vec::new();
        v.push(Box::new(123));
        v.push(Box::new("anu"));
        v.push(Box::new(String::from("nmnm")));
        v.push(Box::new(Custom));
        Self {
            field1: Box::new(Dep),
            field2: v,
        }
    }
}
```

Rust memiliki pendekatan OOP yang berbeda dengan kebanyakan bahasa OOP, dimana tidak ada inheritance, dan kebanyakan variable adalah value, bukan object seperti di bahasa OOP. Beberapa pattern juga akan sangat berbeda jika diterapkan OOP dengan Rust. Trait object juga bisa di-inject ke dalam smart pointer jenis lain, sesuai kebutuhan. 