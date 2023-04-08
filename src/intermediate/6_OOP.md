# Object Oriented Programming in Rust #

OOP memiliki banyak definisi seiring waktu. Bermula dari Simula yang mengenalkan konsep object, program disusun sedemikian rupa dari sekumpulan object-object yang dibangung dari class, attributes, dan virtual procedures(methods). Selanjutnya oleh Smalltalk menambahkan konsep message passing, dimana object akan menerima suatu *message* melalui method-method nya, dan menyesuaikan state berdasarkan itu. Java datang dengan konsep yang mirip-mirip dan dijadikan acuan dalam OOP yaitu memiliki:
- Encapsulation: Information hiding terhadap detail implementasi dengan hanya memberi akses API lewat public API.
- Inheritance: Ekstensi object memberikan relasi "is a".
- Polymorphism: Deklarasi common behaviour antar multiple objects. Diimplementasikan dengan menggunakan Vtable, dynamically checked.

## Encapsulation ##
Secara gamblang, encapsulation merupakan cara meng-abstraksi API sehingga user tidak perlu melihat daleman API dan hanya berinteraksi melalui exposed interface. Di sebagian bahasa OOP hal ini dilakukan menggunakan access modifier seperti *public* dan *private*. Hal ini untuk mencapai *loosely coupling* pada design API sehingga implementor bisa fokus terhadap implementasi, dan client/user dari API bisa tetap menggunakan API yang sama tanpa peduli detail implementasi.

Pada Rust, ada beberapa cara untuk mencapai ini, diantaranya:
- Access modifier `pub`, semua komponen bersifat private, dengan `pub` membuatnya bisa di-akses oleh caller.
- Menggunakan crate library `lib.rs` untuk mengekpose api-api yang bisa digunakan oleh client.
- Menggunakan module untuk meng-segmentasi kode sedemikian rupa sehingga pengelompokkan API lebih terorganisir dan koheren.
- Menggunakan inherent methods dengan beberapa public methods, dan private methods.
- Menggunakan traits dengan shared custom types, methods, dan default types. Traits juga digunakan sebagai bound untuk type parameter API.

## Inheritance ##
Rust tidak memiliki inherintance layaknya kebanyakan bahasa OOP lainnya. Akan tetapi Rust menerapkan komposisi yaitu dengan meng-*embed* tipe data/struct/enum ke dalam fields. Inheritance sendiri memiliki masalah dalam dunia OOP karena cenderung menambah kompleksitas yang tidak diperlukan. Juga mempersulit *maintainance* dari suatu kode karena sedikit perubahan akan mengacaukan semua childs yang inherit suatu class, cenderung menjadi *spaghetti*. Selain itu juga bisa menimbulkan masalah yang disebut *diamond problem*. Inheritance bukan pilihan utama ketika membuat kode yang loosely coupled dan juga enkapsulasi lebih cenderung menggunakan interface(*composition*) ketimbang class inheritance. Rust memiliki relasi antar type yang disebut dengan *Subtyping*.

## Polymorphism ##
Pada bahasa OOP umumnya polymorphism dilakukan menggunakan *interface*. Interface merupakan salah satu jenis inheritance yang bersifat *has-a* untuk mencapai composition.

Rust mengimplementasikan konsep polymorfisme melalui dua cara yaitu: *generic* dan *trait object*. Kita telah membahas generic pada pembahasan sebelumnya yaitu bersifat statis dan diperiksa pada saat compile-time. Akan tetapi pendekatan generics ini memiliki kekurangan yaitu hanya memiliki satu instance dalam satu waktu. Selain itu tipe data yang akan digunakan pada generic/type parameter harus diketahui size nya pada saat compile-time. Berangkat dari kekurangan inilah kita membutuhkan polymorfisme yang bersifat lebih dinamis dan semua tipe bisa di-inject pada saat runtime, sehingga tidak perlu di-specify semua pada saat run-time, cara ini disebut *trait object*.

Perbedaan generics dan trait object:
- Generic resolve at *compile-time*, while trait object resolve at *run-time*.
- Hanya ada 1 instance tipe data pada tipe yang memiliki generic parameter, sedangkan trait object bisa memiliki menampung berbagai tipe selama tipe tersebut mengimplementasi deklarasi trait object tersebut.
- Generic tidak memiliki runtime cost karena statically dispatched, sedangkan trait object memiliki runtime cost karena dynamically dispatched(seperti penggunaan vtable yang menampung semua pointer ke fungsi/methods yang digunakan).

# OOP Implementations in Rust #

## States ##
States adalah data yang berubah-ubah sepanjang program berjalan. Data ini biasanya disebut attributes di dalam bahasa OO yang mana states transitions nya dikendalikan oleh method-method dari object tersebut. Rust secara default bersifat *immutable*, karena kecenderungan functional programming. Akan tetapi kita bisa membuat sifat statefulness dari Rust ini dengan menggunakan keyword `&mut self` pada deklarasi method. States merupakan efek dari behaviour object terhadap data yang dikandung object.
```rust
#[derive(Debug)]
pub struct Object {
    field1: String,
    field2: i32,
    field3: u64,
}

impl Object {
    pub fn new(field1: String, field2: i32, field3: u64) -> Self {
        Object {
            field1,
            field2,
            field3,
        }
    }

    pub fn method1(&mut self) {
        self.field1.push_str("new str");
    }

    pub fn method2(&mut self) {
        self.field2 += 1;
    }

    pub fn method3(&mut self) {
        self.field3 += 340;
    }

    pub fn print(&self) {
        println!("{:?}", self);
    }
}
```

## Encapsulation ##
Encapsulation bisa memanfaatkan access modifier `pub` untuk public access, dan memanfaatkan komponen-komponen yang accessible lewat path `::` dan method.
Contoh:
```rust
// encapsulation
#[derive(Debug)]
struct A {
    field1: String,
    field2: i32,
}

impl A {
    fn new() -> Self {
        A {
            field1: String::from("this A"),
            field2: 123,
        }
    }
    fn method_a(&self, n: u64) {
        println!("A: {n}")
    }
}

// kita meng-enkapsulasi A ke dalam B, sehingga A tidak perlu di-ekpose ke user.
#[derive(Debug)]
pub struct B {
    a: A,
    field3: u64,
}

impl B {
    pub fn new() -> Self {
        B {
            a: A::new(),
            field3: 234,
        }
    }
    pub fn method_b(&self) {
        println!("printing B");
        self.a.method_a(self.field3);
    }

    // data A di-enkapsulasi ke dalam B, sehingga bisa di akses tanpa akses ke A.
    pub fn method_b_2(&self) -> (&str, i32) {
        (self.a.field1.as_str(), self.a.field2)
    }
}
```

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