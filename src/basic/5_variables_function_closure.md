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
Closure adalah anonymous function yang merupakan deklarasi fungsi tanpa nama yang biasa digunakan sebagai tipe parameter untuk menerima fungsi(pure). Closure di dalam Rust secara default menangkap value-value sekitar secara reference(borrowed).
```rust
let s = String::from("lskdmfsdf");
let a = || {
    println!("{}", s); // s is captured by reference(borrowed) implicitly by closure
    println!("Im printed from closure!");
};
a();
println!("{}", s); // still valid because s is only borrowed
```
Untuk menangkap values sekitar tanpa borrowing, bisa menggunakan keyword *move* sebelum opening bar.
```rust
let s = String::from("lskdmfsdf");
let b = move |x: i32| -> i32 {
    println!("{}", s); // because of keyword `move` above, this string is moved here so cannot be mentioned/referenced below anymore.
    x * x
};
b(123);
println!("{}", s); // cannot call `s` here because already moved into b closure scope.
```
Untuk tipe-tipe yang bersifat *copy-able* seperti tipe primitif, ketika melakukan move, value di-copy ke dalam closure sehingga masih tetap bisa di panggil setelah closure. Move semantic benar-benar diterapkan untuk value yang bersifat *clone-able*.