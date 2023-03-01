# Functional Programming pada Rust #

Rust memiliki berbagai macam paradigma pemograman yang dapat dicampurkan ke dalam kode, yaitu: Procedural/Imperative dan Functional/Declarative. Procedural merupakan pendekatan bahasa pemograman yang paling awal muncul memiliki pengaruh langsung dari konsep Turing machine itu sendiri. Selain itu juga bahasa assembly juga secara keseluruhan bersifat imperative dan procedural. Pendekatan ini menekankan penulisan kode program secara berurutan step-by-step dari atas ke bawah dengan perubahan state di antaranya.

Pendekatan kedua yang muncul yaitu Functional yang memiliki pengaruh dari Lambda Calculus oleh Alonzo Church. Pendekatan kedua ini menggunakan fungsi sebagai dasar komputasi yang bisa dikombinasikan dan digabungkan menghasilkan abstraksi yang lebih tinggi untuk melakukan komputasi dengan menekankan *immutability* dari data.

Rust dengan type system nya bisa meng-*achieve* pendekatan functional dengan memanfaatkan 3 komponen utama:
- [Generic types](../basic/15_generic.md)
- [Traits](../basic/14_trait.md)
- [Closures](../basic/5_variables_function_closure.md#closures)

Rust mendapatkan polymorphic type parameters secara static dan strong oleh generics dan traits, serta dapat mengembangkan komputasi menggunakan closures.
Kita telah membahas ketiga di atas pada pembahasan sebelumnya. Pada kali ini kita akan membahas kombinasi dari 3 hal di atas menjadi sesuatu yang lebih abstrak disebut Higher-order functions. Sebelum itu kita akan membahas beberapa istilah yang berkaitan dengan higher-order function ini, yaitu:

## First-order function ##
Adalah fungsi yang bisa diberlakukan layaknya value yang bisa dijadikan sebagai parameter/tipe data.
Rust memiliki konsep ini menggunakan closures:
```rust
let closure = |x| x * x; // can be inferred at calling site
println!("{:?}", closure(123));
```
Value `closure` merupakan contoh first-order function di dalam rust, menggunakan `|params|` sebagai scope parameters.

Selain itu, ketika ingin menggunakan value tersebut sebagai parameters, terdapat 3 jenis closures yang di-define menggunakan functional traits: `FnOnce`, `FnMut`, dan `Fn` yang telah dibahas pada [jenis-jenis closure](../basic/5_variables_function_closure.md#jenis-jenis-closure)