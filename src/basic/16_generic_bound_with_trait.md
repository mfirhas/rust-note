# Generic Bound with Trait #

Ketika ingin membatasi tipe-tipe yang dibolehkan pada generic, kita bisa menggunakan `trait` sehingga tipe-tipe yang bisa dimasukkan ke dalam generic type hanya tipe yang sudah mengimplementasikan `trait` bersangkutan.

Generic trait bound bisa dideklarasikan pada tempat-tempat dimana generic berada.

Berikut beberapa tempat mendeklarasi trait bound pada generic type:

## Function Params ##
```rust
use core::fmt::Display;

fn my_format<T: Display>(param: T) {
    println!("-->: {}", param);
}
```
Trait `Display` diimport dari module `core::fmt`. Trait ini merupakan trait untuk tipe-tipe yang bisa di-print ke stdout atau stderr. Semua tipe-tipe primitive sudah memiliki implementasi ini built-in, beberapa tipe non-primitive lain juga sudah. Untuk tipe yang belum kita bisa implement sendiri trait tersebut.

Trait `Display` dibutuhkan karena tipe T akan digunakan di dalam `println!()` yang membutuhkan data dengan tipe yang implement `Display` trait.

- Multiple generics:
```rust
fn function_generic_multiple_bounds<T: Display + Clone>(param: T) {
    println!("{}", param.clone());
}

fn function_multiple_generic_multiple_bounds<T: Display + Clone, U: Debug>(param1: T, param2: U) {
    println!("{} -> {:?}", param1.clone(), param2)
}
```

- Memanggil multiple generics:
```rust
// memanggil dengan definisi tipe
function_generic_multiple_bounds::<&str>("this is testing"); 
function_multiple_generic_multiple_bounds::<&str, i64>("test", 123);

// memanggil dengan tipe data infered
function_generic_multiple_bounds("this is testing");
function_multiple_generic_multiple_bounds("test", 123);
```

- Multiple generics dengan `where`:
```rust
fn function_generic_trait_bounds_where<T, U, V>(param1: T, param2: U, param3: V) -> String
where
    T: Clone + Display,
    U: Debug,
    V: Display,
    String: From<V>
{
    println!("{} {:?}", param1, param2);
    String::from(param3)
}
```

- Memanggil multiple generics dengan `where`:
```rust
let resp: String = function_generic_trait_bounds_where("test", 123, "this");
println!("{}", resp);
```


## Struct dan Enum beserta associated functions dan methods ##
### Struct ###
```rust
#[derive(Debug)]
struct MyStruct<T, U> {
    field1: T,
    field2: U,
}

impl<T: Display, U: Debug + Display> MyStruct<T, U> {
    pub fn method1(param1: T, param2: U) {
        println!("{}, {}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{}, {}", self.field1, self.field2);
    }
}
```

- Implementasi menggunakan `where` clause:
```rust
impl<T, U> MyStruct<T, U> where
    T: Display,
    U: Debug + Display
{
    pub fn method1(param1: T, param2: U) {
        println!("{}, {}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{}, {}", self.field1, self.field2);
    }
}
```

- Memanggil:
```rust
MyStruct::<i32, &str>::method1(123, "test");
MyStruct::method1(123, "test"); // infered
```

### Enum ###
```rust
#[derive(Debug)]
pub enum MyEnum<T,U> {
    Value1(T),
    Value2(U),
}

impl<T: Debug + Display, U: Debug + Display> MyEnum<T,U> {
    pub fn method1(param1: T, param2: U) {
        println!("{:?}, {:?}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{:?}", self);
    }
}
```

- Implementasi menggunakan `where` clause:
```rust
impl<T, U> MyEnum<T,U> where
    T: Debug + Display,
    U: Debug + Display
{
    pub fn method1(param1: T, param2: U) {
        println!("{:?}, {:?}", param1, param2);
    }

    pub fn method2(&self) {
        println!("{:?}", self);
    }
}
```

- Memanggil:
```rust
MyEnum::<i32, f32>::Value1(9999).method2();
```

Untuk `Struct` dan `Enum` trait bounds lebih baik di deklarasikan untuk `impl` karena filter tipe berlaku ketika ada operasi terhadap data generic tersebut. Jika hanya struct dan enum saja, maka tidak berarti apa-apa bounds yang diberikan karena kita tidak melakukan apa-apa terhadap data tersebut.

## Trait ##
```rust
trait MyTrait<T: Display, U: Display + Clone> {
    fn method1(param: T);
    fn method2(&self, param: U);
    fn method3(&self, param: T) -> U;
}

struct MyStruct4 {
    field1: String,
    field2: i32,
}

impl MyTrait<i32, String> for MyStruct4 {
    fn method1(param: i32) {
        println!("{}", param);
    }

    fn method2(&self, param: String) {
        println!("{} : {}", self.field2, param.clone());
    }

    fn method3(&self, param: i32) -> String {
        format!("field1: {}, field2: {}, param: {}", self.field1, self.field2, param)
    }
}
```

- Pemanggilan kode di atas:
```rust
MyStruct4::method1(123);
let my_struct4 = MyStruct4 {
    field1: String::from("this"),
    field2: 234,
};
my_struct4.method2(String::from("lskdmf"));
<MyStruct4 as MyTrait<i32, String>>::method1(4999);
let resp = my_struct4.method3(7890);
println!("{}", resp);
```

- Implementasi trait dengan generic implementor:
```rust
trait MyTrait<T: Display, U: Display + Clone> {
    fn method1(param: T);
    fn method2(&self, param: U);
    fn method3(&self, param: T) -> U;
}

struct MyStruct4<T, U> {
    field1: T,
    field2: U,
}

impl<T: Display, U: Clone + Display> MyTrait<i32, String> for MyStruct4<T, U> {
    fn method1(param: i32) {
        println!("{}", param);
    }

    fn method2(&self, param: String) {
        println!("{} : {}", self.field2, param.clone());
    }

    fn method3(&self, param: i32) -> String {
        format!("field1: {}, field2: {}, param: {}", self.field1, self.field2, param)
    }
}
```

- Pemanggilan kode di atas:
```rust
MyStruct4::<String, i32>::method1(123);
let my_struct4 = MyStruct4 {
    field1: String::from("this"),
    field2: 234,
};
my_struct4.method2(String::from("lskdmf"));
<MyStruct4<String, i32> as MyTrait<i32, String>>::method1(4999);
let resp = my_struct4.method3(7890);
println!("{}", resp);
```

Untuk `trait` bounds dapat ditaruh pada deklarasi trait, sehingga bound ini akan diterapkan pada setiap tipe yang mengimplementasikan trait tersebut.

Perbedaan generic bounds pada `impl` antara trait dengan inherent adalah dengan trait kita bisa deklarasi generic return type, yang akan didefinisikan tipenya saat implementasi oleh suatu tipe. Seperti contoh di atas, fungsi `fn method3(&self, param: T) -> U;` memiliki generic return dan tipe dari return didefinisikan pada saat implementasi `impl MyTrait<i32, String> for MyStruct4` sehingga return type menjadi String.