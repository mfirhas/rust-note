# Variables ,Functions, dan Closures #

Variables di dalam rust secara default bersifat *immutable* yaitu tidak bisa diubah atau diassign ulang. *Immutability* merupakan pendekatan fungsional dari rust untuk menghindari *side-effects* di dalam program atau unexpected behaviour lainnya seperti race condition dan data race. Immutability juga memudahkan melakukan *tracing* data di dalam code program tanpa khawatir penggunaan selanjutnya di sisi lain code program.
Variables dideklarasi di-dalam function/method atau disebut juga local variables.

## Local Variable ##
Variable yang dideklarasi hanya di dalam fungsi atau method, dan akan dihapus ketika stackframe fungsi dihapus.
Naming convention local variable menggunakan *snake_case*.
Berikut contoh deklarasi local variable:
```rust
let a = 123;
let b: i32 = 123;
let c = "literal string";
let d: String = "Object String".to_string();

// deklarasi mutable
let mut a = 123;
a = a + 1; // 124

let mut s = String::from("test");
s.push_str("anu");
println!("{}", s); // testanu
```
Pada contoh di atas, terdapat 4 jenis deklarasi variable. Variable `a` tidak memiliki deklarasi tipe karena Rust dapat meng-*infer* tipe data tersebut saat compile time. Variable `b` memiliki tipe data setelah colon `:`. Variable `c` merupakan jenis string literal atau reference string. Variable `d` merupakan jenis string *owned* dimana value string literal harus diubah ke owned menggunakan method `to_string()`.

## Shadowing ##
Shadowing adalah ketika local variable lama dioverwrite secara tipe dan value. Contoh:
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

## Static ## 
Memiliki kemiripan dengan `const`, perbedaan terletak pada static memiliki alamat memory ketika dikompilasi. Ketika variable static dipanggil maka ada proses dereference terhadap value yang ada di alamat memori dari variable static tersebut. Perbedaan lainnya adalah static dapat menjadi mutable pada saat runtime. Mutability static pada saat runtime hanya bisa dilakukan di dalam blok `unsafe`. Contoh deklarasi static:
```rust
static a: i32 = 123;
static b: &str = "anu";
println!("{:p}", a);
println!("{:p}", b);
```
Bukti bahwa static memiliki alamat memori adalah kita bisa mendapatkan alamat memori tersebut dengan cara di atas. Sementara kita tidak bisa melakukan hal itu terhadap constant karena constant tidak memiliki alamat memori.

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
let add_one_v4 = |x| x + 1  ;
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

