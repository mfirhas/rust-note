# Associated Item #

Associated Items merupakan tipe data yang memiliki kaitan dengan tipe, constants, fungsi, atau method. Hal ini juga bisa digunakan sebagai salah satu metode untuk *"namespacing"* di dalam Rust.

Associated items bisa dilakukan di dalam traits atau type implementations.

Associated items langsung pada struct/enum/UDT disebut `inherent`

Associated items secara keseluruhan bisa dibagi kedalam 3 jenis:
- Associated functions and methods
- Associated types
- Associated constants

## Associated functions and methods ##
Fungsi dan method yang berhubungan dengan suatu type. Jenis deklarasi seperti ini disebut *inherent functions/methods*. Type-type yang bisa mendeklarasi *inherent functions/methods* adalah non-primitive types dan harus dideklarasikan di dalam 1 crate yang sama dengan type bersangkutan. Jika selain dari itu, gunakan trait.

### Associated Functions & Methods ###
Di Rust, perbedaan antara **function** dan **method** adalah:
- **function**: *standalone*, *pure* dan tidak memiliki objek yang berkait, bisa dideklarasikan tanpa `impl`.
- **method**: berkait dengan suatu objek(self) dengan berbagai macam tipe(struct, enum, dll), hanya bisa dideklarasikan dengan `impl`.

Contoh function:
```rust
// free function
fn function_name() {
    // body
}

struct Type;

// associated function
impl Type {
    fn function_name() {

    }
}

// cara memanggil associated function
Type::function_name();
```

Contoh method:
```rust
struct Type {
    id: i32;
}

impl Type {
    // self merupakan parameter sekaligus argumen yang merepresentasikan tipe terkait dengan method.
    fn method_name(&self) {
        println!("{}", self.id);
    }
}

// cara memanggil method
let t = Type {
    id: 123,
}
t.method_name()
```
### Self type pada method ###
Setiap method harus diawali oleh parameter sekaligus argumen pertama berupa `self`. Terdapat beberapa jenis/bentuk tipe `self` yaitu:

|   1st form                                  | ref with lifetime     | Short form(with lifetime) |
|---------------------------------------------|-----------------------|---------------------------|
| self: Self                                  | self                  |                           |
| self: &Self                                 | self: &'a Self        | &'a self                  |
| self: mut Self                              | mut self              |                           |
| self: &mut Self                             | self: &'a mut Self    | &'a mut self              |
| self: Box<Self>                             |                       |                           |
| self: Rc<Self>                              |                       |                           |
| self: Arc<Self>                             |                       |                           |
| self: Pin<&Self>                            |                       |                           |
| self: `<MyType as MyTrait>::associated_type`|                       |                           | 

*Note: untuk `self: <MyType as MyTrait>::associated_type`, `associated_type` harus refer ke tipe implementor, tidak bisa ke tipe lainnya.*

Contoh inherent functions/methods pada struct:
```rust
struct MyStruct {
    a: i32,
    b: String,
}

impl MyStruct {
    pub fn function1() -> i32 {
        123
    }

    pub fn function2(&self) -> String {
        String::from("anu")
    }

    fn function3() -> f32 {
        234_f32
    }
}
```

Contoh memanggil inherent functions/methods pada struct:
```rust
let resp = MyStruct::function1();
println!("{}", resp);
println!("{}", MyStruct::function3());
let s = MyStruct {
    a: 123,
    b: String::from("lskmdf"),
};
println!("{}", s.function2());
```

Contoh inherent functions/methods pada enum:
```rust
enum MyEnum {
    A(String),
    B(i32),
    C(f32),
}

impl MyEnum {
    pub fn function1() -> i32 {
        123
    }

    pub fn function2(&self) -> String {
        match *self {
            MyEnum::A(ref s) => s.clone(),
            _ => String::from("null")
        }
    }
}
```

Contoh memanggil inherent functions/methods pada enum:
```rust
println!("{}",MyEnum::A("halo".to_string()).function2());
println!("{}",MyEnum::B(123).function2());
```

### Associated Traits ###
```rust
trait MyTrait {
    fn method1() -> i32;
    fn method2(&self) -> String;
}

struct MyStruct {
    a: i32,
    b: String,
}

impl MyTrait for MyStruct {
    fn method1() -> i32 {
        123
    }

    fn method2(&self) -> String {
        self.b
    }
}
```

Berikut 4 cara memanggil associated trait:
```rust
// Karena function method3() mengembalikan tipe `Self`, maka memanggil function tersebut dari `MyTrait` akan secara otomatis memanggil implementasi oleh `MyStruct`
// karena MyStruct dijadikan tipe data return oleh variable penerima.
let _: MyStruct = MyTrait::method3();

// Wildcard akan membaca return method3 yang `Self` dan mencocokkan dengan tipe data variable penerima sebelah kiri.
let _: MyStruct = <_ as MyTrait>::method3();

// Implementor sudah didefine ketika memanggil method3 dengan menggunakan Fully Qualified Syntax
let _ = <MyStruct as MyTrait>::method3();

// Langsung menggunakan tipe implementor dengan syarat tipe trait sudah dimasukkan ke dalam scope caller berada.
let _ = MyStruct::method3();

// sebagai contoh, jika dipanggil dari module lain:
// MyTrait harus dibawa ke dalam scope module untuk dapat memanggil method yang diimplement oleh MyStruct
mod a {
    use super::{MyStruct, MyTrait};

    fn sdf() {
        let d = MyStruct::method3();
    }
}
```

## Associated Types ##
Associated Types hanya dideklarasi di dalam `trait` dan didefinisikan di dalam implementor. 

```rust
trait MyTrait {
    type Error: Display + Debug + Clone;

    fn method1() -> i32;
    fn method2(&self) -> String;
    fn method3() -> Self;
    fn method4() -> Result<(), Self::Error>;
}

impl MyTrait for MyStruct {
    type Error = String;

    fn method1() -> i32 {
        123
    }

    fn method2(&self) -> String {
        self.b.clone()
    }

    fn method3() -> Self {
        MyStruct { a: 123, b: "sdf".to_string() }
    }

    fn method4() -> Result<(), Self::Error> {
        return Err("error bung".to_string());
    }
}
```

Ketika memanggil tipe tersebut:
```rust
println!("{}", MyStruct::method4().unwrap_err());
```

## Associated Constant ##
Sama dengan associated type, constants juga bisa di-declare pada trait, dan di-definisikan pada implementor.
Hal yang beda adalah kita bisa memberi default value pada constant yang di-declare di trait bersangkutan. Jika kita memberi default value pada constant trait,
dan mendefinisikannya juga pada implementor, maka value pada implementor akan meng-overwrite default value pada trait.
Associated Constant bisa dideklarasi dan didefinisikan langsung pada `impl` tanpa trait.

```rust
impl MyStruct {
    pub const C: &str = "nganu";
}

trait MyTrait {
    const D: i32;
    // const D: i32 = 0; // default value, 
    type Error: Display + Debug + Clone;

    fn method1() -> i32;
    fn method2(&self) -> String;
    fn method3() -> Self;
    fn method4() -> Result<(), Self::Error>;
}

impl MyTrait for MyStruct {
    const D: i32 = 123; // -> will overwrite default value in trait if declared
    type Error = String;

    fn method1() -> i32 {
        123
    }

    fn method2(&self) -> String {
        self.b.clone()
    }

    fn method3() -> Self {
        MyStruct { a: 123, b: "sdf".to_string() }
    }

    fn method4() -> Result<(), Self::Error> {
        println!("const: {}", Self::D);
        return Err("error bung".to_string());
    }
}
```
Ketika ingin memanggil, kita tidak bisa memanggil constant langsung dari trait, harus melewati type implementor.

Ketika memanggil:
```rust
println!("{}", MyStruct::C);
println!("{}", MyStruct::D);
println!("{}", <MyStruct as MyTrait>::D);
```

## Kesimpulan
User Defined Type(struct, enum, etc) implementation disebut sebagai **inherent** implementation.

UDT inherent implementations bisa memiliki:
- public & private associated constants
- public & private associated functions
- public & private associated methods

Trait declarations bisa memiliki:
- default associated constants
- associated constants
- default associated functions implementation 
- associated functions
- default associated methods implementation
- associated methods
- associated types (along with additional trait bounds(optional))

Visibility dari trait items are following trait visibility.

Non-default associated items must be defined by implementor.

Ketika sebuah UDT implement sebuah trait, ketika struct tersebut digunakan, jika method yang dipanggil memiliki nama yang sama antara inherent dan trait implementations, maka secara *default* akan menggunakan *inherent* implementations.

```rust
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct Struct {
    pub field_1: i32,
    pub field_2: String,
}

impl Struct {
    // public inherent associated constants
    pub const CONSTANT: &str = "haha";

    // private inherent associated constants
    const CONSTANT_2: &str = "hehe";

    // CANNOT has inherent associated type, unstable
    // type Err;

    // public inherent associated function
    pub fn function() {
        ()
    }

    // public inherent associated consuming/moving method
    pub fn method_1(self) -> String {
        self.field_2
    }

    // public inherent associated borrowing method
    pub fn method_2(&self) -> i32 {
        self.field_1
    }

    // private inherent associated mutably-borrowed method
    fn method_3(&mut self) {
        self.field_1 = self.field_1 + 5;
    }

    // public inherent associated mutable borrowing method
    pub fn method_4(&mut self) -> i32 {
        self.method_3();
        self.field_1
    }
}

pub trait Trait {
    // all visibilities follow trait visibility

    // associated constant without default value
    const T_CONSTANT: &'static str;

    // associated constant with default value
    // it can be override by implementor if they redefine it
    const T_CONSTANT_DEFAULT: &'static str = "hihi";

    // associated type (not used), all associated types must be used.
    // type Unused;

    // associated type (without traits bounds)
    type Val;

    // associated type (with trait bounds)
    // any implementor of this trait must define Err with satisfy such trait bounds
    type Err: Display + Debug;

    // associated function with default
    fn function_default() {
        println!("function_default()");
        ()
    }

    // associated function
    fn function();
    
    fn method_default_1(&self)
    where
        Self: Debug,
    {
        println!("{:#?}", self);
    }

    fn method_1(&self) -> &Self::Val;

    fn method_2(&self) -> Result<Self::Val, Self::Err>;
}

impl Trait for Struct {
    const T_CONSTANT: &'static str = "T_CONSTANT in Struct's Trait implementation";

    type Val = i32;

    type Err = String;

    fn function() {
        println!("trait function()");
        ()
    }

    fn method_1(&self) -> &Self::Val {
        println!("trait method_1");
        &self.field_1
    }

    fn method_2(&self) -> Result<Self::Val, Self::Err> {
        println!("trait method_2");
        Ok(self.field_1)
    }
}

fn main() {
    let mut s = Struct {
        field_1: 123,
        field_2: String::from("This is String"),
    };

    // default using inherent implementations
    println!("{}", Struct::CONSTANT);
    println!("{:?}", s.method_2());
    println!("{:?}", s.method_4());
    println!("{:?}", s.method_2());
    println!("{:?}", s.method_1());
    println!("{:?}", Struct::function());

    println!("---");

    let mut s = Struct {
        field_1: 234,
        field_2: String::from("This is String 2"),
    };
    // use fully qualified syntax to use trait implementations when facing same name.
    println!("{:?}", <Struct as Trait>::T_CONSTANT_DEFAULT);
    println!("{:?}", <Struct as Trait>::T_CONSTANT);
    println!("{:?}", <Struct as Trait>::function_default());
    println!("{:?}", <Struct as Trait>::function());
    println!("{:?}", <Struct as Trait>::method_default_1(&s));
    println!("{:?}", <Struct as Trait>::method_1(&s));
    println!("{:?}", <Struct as Trait>::method_2(&s));
    println!("{:?}", <Struct as Trait>::method_2(&s).unwrap());
}
```