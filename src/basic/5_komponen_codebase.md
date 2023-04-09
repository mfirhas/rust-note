# Komponen Codebase #

Kita akan membahas komponen codebase yang merupakan segmentasi dari program Rust:
`Workspace -> Packages -> Modules -> Constants/Statics/Types/Associated Items`

Sebelum kita membahas beberapa komponen pada Rust program, kita akan membahas beberapa konsep yang berkaitan dengan komponen tersebut.

## Immutability vs Mutability ##
Setiap variables dan statics pada Rust bersifat *immutable* secara default. Ini merupakan salah satu pendekatan functional programming dimana untuk menghindari side-effects pada data. Immutability menyebabkan data tidak bisa di-*assign* dengan value lain dan tidak bisa diubah. 
Jika ingin memberikan mutability pada variable, bisa menggunakan keyword `mut` atau `&mut` untuk mutable borrow.
Mutability hanya berlaku untuk variables dan statics. Untuk statics jika ingin melakukan mutasi, harus di dalam `unsafe` block.

## `let` binding ##
Merupakan keyword untuk mendeklarasi variable dan memberikan lifetime pada variable tersebut sampai berpindah/akhir scope. Karena semua variable pada rust bisa di-referensikan/borrowed, maka semua variable memiliki lifetime. Ketika kita mendeklarasi `let` untuk menciptakan variable dengan nama, maka kita sudah mendeklarasikan lifetime dari reference dari variable tersebut, sepanjang lifetime dari deklarasi `let`. 

Kita bisa mendeklarasi reference pada data literal langsung tanpa `let` binding, akan tetapi ini menyebabkan reference tersebut hanya memiliki lifetime sepanjang statement assignment tempat literal reference tersebut dibuat. [Ref](https://users.rust-lang.org/t/temporary-value-is-freed-at-the-end-of-this-statement/84667/4?u=mfathirirhas) , hal ini hanya berlaku untuk *mutable reference*.

Contoh:
```rust
{
    let mut orphan_reference = &123;
    orphan_reference = &900;
    println!("{:?}", orphan_reference);
    dbg!(orphan_reference);

    // failed karena mutable orphan reference without let binding live only as long as its statement.
    // let mut orphan_reference = &mut 123;
    // orphan_reference = &mut 900;
    // println!("{:?}", orphan_reference);
    // dbg!(orphan_reference);

    let binding: i32 = 123;
    let borrow = &binding;
    dbg!(binding);
    dbg!(borrow);

    let binding: String = "123".to_owned();
    let borrow = &binding;
    println!("{:?}", binding);
    println!("{:?}", borrow);
    dbg!(&binding);
    dbg!(borrow);
}
```

Kita akan membahas lebih detail tentang komponen-komponen yang ada di dalam codebase Rust. Secara umum dapat terbagi di antaranya:
- Variables (local variables)
- Statics (global variables)
- Constants
- Functions
- Closures
- Types (User Defined Types)
- Associated Types/Items

Semua komponen di atas berada di dalam module(`mod`).

## Variable ##
Variable di dalam Rust hanya bisa dideklarasi di dalam scope function. Variable ini bersifat local dan akan dihapus setelah function return/exit.
Naming convention local variable menggunakan *snake_case*.
Berikut contoh deklarasi local variable:
```rust
let a = 123;
let b: i32 = 123;
let c = "literal string";
let d: String = "Object String".to_string();
let e: = &123;

// deklarasi mutable
let mut a = 123;
a = a + 1; // 124

let mut s = String::from("test");
s.push_str("anu");
println!("{}", s); // testanu
```
Pada contoh di atas, terdapat 4 jenis deklarasi variable. Variable `a` tidak memiliki deklarasi tipe karena Rust dapat meng-*infer* tipe data tersebut saat compile time. Variable `b` memiliki tipe data setelah colon `:`. Variable `c` merupakan jenis string literal atau reference string. Variable `d` merupakan jenis string *owned* dimana value string literal harus diubah ke owned menggunakan method `to_string()`.

## Reference Variable (`&`) ##
Merupakan variable yang berisi address dari variable yang direferensikan.
Beberapa kegunaan reference variable:
- Menyediakan data secara *read-only* menggunakan *immutable* reference tanpa memindahkan *ownership* data jika *clonable*
- Memberikan *side-effect* kepada fungsi menggunakan *mutable* reference tanpa memindahkan *ownership* data jika *clonable*
- Mem-*passing* data antar fungsi/method tanpa melakukan *copy*/*clone* terhadap value, sehingga yang di-*copy*/*clone* hanyalah address dari data(lebih murah).

## Shadowing ##
Shadowing adalah ketika local variable lama dioverwrite dengan tipe dan value baru. Contoh:
```rust
let a = 123; // i32
let a = "from i32 to string"; // &str
let a = "from string to owned string".to_string(); // String
```
Variable `a` dapat dideklarasikan kembali berulang-ulang dengan tipe yang berbeda. Value yang digunakan adalah yang terakhir kali dideklarasikan.

## Constant ##
Constant juga immutable by default dan tidak bisa dijadikan mutable. Deklarasi constant mengharuskan menggunakan tipe data. Constant harus dideklarasikan menggunakan value yang dapat dikomputasi pada saat kompilasi. Jika value butuh alokasi heap, maka tidak bisa di-*assign* ke constant . Constant dapat berisi ekspresi non-final dan dikomputasi pada saat kompilasi. Constant dievaluasi pada saat kompilasi dan ditaruh ke dalam binary program, sehingga tidak memiliki address di memory. Constant valid selama program berjalan. Contoh deklarasi constant:
```rust
const a: i32 = 123;
const b: &str = "anu";
```

Masih banyak jenis constant evaluated values/expressions lainnya yang akan dibahas berikutnya.

## Static ## 
Memiliki kemiripan dengan `const`, perbedaan terletak pada static memiliki alamat memory ketika dikompilasi. Ketika variable static dipanggil maka ada proses dereference terhadap value yang ada di alamat memori dari variable static tersebut. Perbedaan lainnya adalah static dapat menjadi mutable pada saat runtime. Mutability static pada saat runtime hanya bisa dilakukan di dalam blok `unsafe`. Contoh deklarasi static:
```rust
static a: i32 = 123;
static b: &str = "anu";
println!("{:p}", a);
println!("{:p}", b);
```
Bukti bahwa static memiliki alamat memori adalah kita bisa mendapatkan alamat memori tersebut dengan cara di atas. Sementara kita tidak bisa melakukan hal itu terhadap constant karena constant tidak memiliki alamat memori.

## Types (User Defined Types) ##
Berikut beberapa tipe data yang bisa di-deklarasi oleh user dan bisa masuk ke dalam bagian namespace:
- `struct`
```rust
struct MyStruct {
    f1: i32,
    f2: String,
}
```
- `enum`
```rust
enum MyEnum {
    F1(i32),
    F2(String),
}
```
- `union`
```rust
#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
```
- Types aliases
```rust
type Integer32 = i32;
type Float64 = f64;
```
Type alias bisa menerima value dengan tipe yang sama dengan alias-nya.

## Types ##
Types disini merupakan user defined types, yang biasanya berupa struct, tuple, enum, dan alias. 
Contoh alias:
```rust
type CustomResult<T> = Result<T, String>

fn get() -> CustomResult<T> {
    Ok(1)
}
fn get_2() -> CustomResult<T> {
    Err("error_message".to_string())
}
```
Type aliases berguna untuk mempersingkat tipe yang panjang.

## Associated types/items ##
Merupakan type custom atau items yang ada di dalam deklarasi inherent implementations seperti `impl X` atau di dalam trait.

Contoh Associated types/items:
```rust
struct Struct;

impl Struct {
    // pub type Item = i32; // as of now it's unstable
    pub const CONSTANT_STR: &str = "test"; // associated constant

    pub fn function() {} // associated function

    // ...
}

trait Trait {
    const C: i32; // associated constant without default value.
    const D: &'static str = "defined"; // associated constant with default value, can be replaced with implementor.
    type CustomResult<'a, T>; // associated type without default value.
    // type CustomResult<'a, T> = Result<T,&'a str>; // associated type with default value, as of now it's unstable.

    fn function();
    fn default_function() {

    }

    fn method(&self);
    fn default_method(&self) {

    }
}

impl Trait for Struct {
    const C: i32 = 123;
    const D: &'static str = "replaced"; // overwrite Trait default value of D
    type CustomResult<'a, T> = Result<T, &'a str>;

    fn function() {
        
    }
    
    // ...
}
```

## Functions ##
Fungsi merupakan unit komputasi paling dasar dengan deklarasi:
```rust
fn function_name(param1: Type1, param2: Type2, ...) -> ReturnType {
    ...
}
```

## Closures ##
Closures merupakan anonymous function atau lambda pada Rust yang digunakan ketika kita ingin menjadikan function sebagai argument pada parameter fungsi lainnya.
Berbeda dengan fungsi biasa di Rust yang tidak bisa meng-*capture* data pada environment bersangkutan, Closures secara default akan menangkap environment sekitar dengan cara borrow/reference.
Berikut contoh deklarasi closures yang valid di dalam Rust:
```rust
// deklarasi closure dengan tipe data.
let add_one_v2 = |x: u32| -> u32 { 
    x + 1
    //...
    //... 
};

// deklarasi tanpa tipe data, dengan curly brackets.
let add_one_v3 = |x| { 
    x + 1;
    //...
    //...
};

// deklarasi tanpa tipe data, tanpa curly brackets.
let add_one_v4 = |x| x + 1;
add_one_v4(5); // return 6
```
Compiler akan secara otomatis meng-infer tipe data dari parameter yang tidak diberi tipe data. Jika body hanya 1 baris, maka block curly brackets bisa dihilangkan.

Closure menangkap data sekitar secara borrow/reference secara default, jika ingin memindahkan ownership(move) dari data tersebut, kita bisa menggunakan move semantics dengan keyword `move` sebelum bar pertama.
```rust
let owned_data = String::from("im owned");
let catch_by_move = move || {
    println!("catched data: {:?}", owned_data);
}
println!("{:?}", owned_data); // will failed because moved into closure
```
Move semantics hanya berlaku untuk tipe data yang bersifat *clone-able*. Untuk data-data yang bersifat static seperti primitive types, move semantics akan meng-*copy* data tersebut kedalam closure sehingga masih valid jika dipanggil lagi.
```rust
let int = 123;
let closure = || {
    let x = 123;
    println!("{}", x);
};
closure();
println!("{}", int);
```

## Jenis-jenis Closure ##
Semua closures pada Rust akan memiliki 1,2 atau 3 dari jenis di bawah ini tergantung bagaimana body closure meng-handle environment sekitar.

### [FnOnce](https://doc.rust-lang.org/std/ops/trait.FnOnce.html) ###
Closure trait paling dasar dimana semua jenis closure akan compatible.
Beberapa sifat `FnOnce`:
- Dipanggil cuma 1 kali.
- Call-by-value, karena value yang dimasukkan ke dalam body closure di-consumed sehingga hanya bisa di-call sekali.
- menerima semua jenis closures(FnMut & Fn) jika ditaruh sebagai parameter input.
  Karena FnOnce merupakan supertype dari semua jenis functional trait yang ada di dalam Rust, sehingga bisa menerima semua child-nya.
Contoh:
```rust
fn main() {
    let string = String::from("string_value");
    do_fn_once(|| {
        println!("do_fn_once {:?}", string);
    });
    println!("{:?}",string);
}

fn do_fn_once<F: FnOnce()>(f: F) {
    f();
    // f(); /// if this is invoked, will failed because FnOnce will moved the `string` into the closure without requiring `move` declaration before closure bar.
}
```

### [FnMut](https://doc.rust-lang.org/std/ops/trait.FnMut.html) ###
Subtype dari `FnOnce`, dengan kriteria:
- Bisa dipanggil lebih dari 1 kali
- Call-by-mutable-reference
- Digunakan ketika ingin meng-capture mutable value by reference.
Contoh:
```rust
fn main() {
    let mut string = String::from("mutate me");
    do_fn_mut(|| {
        string.push_str("|changed|");
    });
    println!("{}", string); // print "mutate me|changed||changed||changed|", because closure invoked 3 times inside `do_fn_mut()`
}

fn do_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
    f();
}
```

### [Fn](https://doc.rust-lang.org/std/ops/trait.Fn.html) ###
Subtype dari `FnMut`, dengan kriteria:
- Bisa dipanggil lebih dari 1 kali
- Call-by-immutable-reference
- Digunakan ketika ingin meng-capture value sekitar by reference berkali2.
Contoh:
```rust
fn main() {
    let string = String::from("borrow me");
    do_fn(|| {
        println!("called from closure: {:?}", string);
    });
    println!("{}", string);
}

fn do_fn<F: Fn()>(f: F) {
    // can be called multiple times since values trapped inside only borrowed(referenced).
    f();
    f();
    f();
}
```

## Function Pointer ##
Kalau closure merupakan trait, maka function pointer merupakan type. Kita tidak perlu mendeklarasi generic dengan bound closure traits untuk mem-*passing* function pointer. Function pointer merupakan ekstensi dari semua function traits, sehingga function pointer bisa diterima di parameter fungsi dengan tipe parameter function traits apapun(FnOnce, FnMut, Fn). Hal ini berguna khususnya ketika kita sudah punya definisi fungsi dan ingin menggunakan fungsi tersebut sebagai value pada fungsi/method lain.

Perbedaan lainnya closure dengan function pointer adalah function pointer tidak bisa menangkap variable sekitar, hanya lewat parameter.

Contoh:
```rust
pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn f(x: i32) -> fn(i32) -> i32 {
    f2
}

fn f2(x: i32) -> i32 {
    123
}

// failed, because return type is covariance so subtype should be passed to supertype,
// but what happen here is we are passing supertype(closure) into subtype(function pointer)
// pub fn multiply(x: i32, y: i32) -> fn(i32) -> i32 {
//     |z| -> i32 { x * y * z }
// }

pub fn accept_fn(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y)
}

pub fn return_fn() -> fn(i32, i32) -> i32 {
    add
}

pub fn return_nested_fn_with() -> fn(i32) -> fn(i32) -> i32 {
    f
}

```