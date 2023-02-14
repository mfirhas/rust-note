# Error Handling #

Error terjadi di dalam *fallible operations*, yaitu operasi-operasi yang memungkinkan terjadi efek samping diluar dugaan dan terjadi pada saat runtime. Hal-hal ini biasanya seperti *IO operations*, index access beyond size, etc. Di dalam Rust terdapat dua jenis eror yaitu: *Recoverable Error* and *Unrecoverable Error*.
- *Recoverable Error* merupakan error karena kesalahan logic atau efek samping dari operasi IO. Error jenis ini dihandle menggunakan tipe data `Result<T,E>` yang merupakan tipe dasar sebuah enum yang menghasilkan salah satu dari dua kemungkinan yaitu `T` untuk berhasil, dan `E` untuk error. Setiap fungsi/method yang memiliki return type `Result` memiliki efek samping/kemungkinan error, sehingga ada penanganan khusus untuk dua kemungkinan return value.
- *Unrecoverable Error* merupakan error karena kesalahan teknis ketika program berjalan, seperti mengakses array diluar size. Error jenis ini akan menghasilkan kondisi disebut panic yang menyebabkan program exit.

## Recoverable Error ##
Merupakan error yang disebabkan oleh efek samping dari program yang biasanya terjadi karena operasi-operasi yang bersifat *fallible* seperti IO atau interaksi dengan users. Error ini tentunya tidak mungkin menyebabkan program *down* karena kita tidak bisa sepenuhnya mengendalikan efek samping itu. Tipe data dasar yang disediakan Rust untuk meng-*handle* error jenis ini adalah enum `Result<T,E>`.
```rust
enum Result {
    Ok(T),  // T adalah tipe data jika proses berhasil dan mengembalikan nilai dengan tipe T
    Err(E), // E adalah tipe data jika proses gagal dan mengembalikan error dengan tipe E
}
```
`Result` Merupakan tipe varian yang memiliki dua kemungkinan yaitu berhasil(`Ok(T)`) atau error(`Err(e)`). Contoh:
```rust
fn fallible_function() -> Result<i32, String> {
    // some process
    // Err(String::from("got error")) // jika gagal
    Ok(1) // jika berhasil
}
```

### Handling Recoverable Error ###
Ketika ingin memanggil fungsi/method yang memiliki kemungkinan error, tentunya kita ingin meng-handle error tersebut. Terdapat beberapa cara menghandle error di dalam Rust diantaranya:
#### **Pattern matching Menggunakan `match`** ####
```rust
let ret = fallible_function();
match ret {
    Ok(t) => println!("val: {}", t),
    Err(e) => panic!("{}", e),
}
```
Pattern matching melakukan destrukturalisasi terhadap tipe data yang di-*match* sehingga kita bisa membaca value-value hasil destruktur dengan menulis signature tipe tersebut. Contoh di atas `fallible_function()` mengembalikan tipe `Result<i32, String>` sehingga berdasarkan definisi tipe enum Result menjadi:
```rust
Result<i32, String> = Ok(i32) | Err(String)
```
Return value merupakan nilai antara i32 jika berhasil, atau String jika gagal/error.
Ketika ingin membaca value dari pattern matching, terdapat beberapa cara membaca:
- **Signature dengan variable**
  Ketika ingin membaca value dari `T` tanpa meng-enumerasi semua kemungkinan value tersebut, cukup dengan mendeklarasikan suatu variable arbitrary seperti:
  ```rust
  match ret {
    // variable `t` bisa diganti menjadi apapun
    Ok(t) => println!("val: {}", t),
    // variable `e` bisa diganti menjadi apapun
    Err(e) => panic!("{}", e),
  }
  ```
  Tipe data dari variable tersebut sesuai dengan tipe data dari deklarasi `Result<i32, String>`.
- **Signature dengan value**
  Ketika ingin memeriksa langsung kepada value yang diinginkan, bisa langsung menulis value yang diinginkan seperti:
  ```rust
  match fallible_function() {
    // kita ingin memastikan bahwa kriteria berhasil hanya jika value dari T adalah 5
    Ok(5) => println!("berhasil"),
    // kita menggunakan wildcard jika kita tidak 
    _ => panic!("panic"),
  }
  ```
  Seperti yang sudah dibahas pada chapter algebraic types bahwa `match` bersifat *exhaustive* sehingga kita bisa menggunakan wildcard `_` untuk mengabaikan semua enumerasi value selain dari yang kita inginkan.

### Pattern Matching menggunakan macro `matches!` ###
`matches!` merupakan macro yang melakukan pengecekkan match langsung terhadap ekspresi yang dimasukkan dengan pattern yang dicocokkan. Dengan ini tidak harus meng-enumerasi semua kemungkinan value, kita hanya ingin memeriksa value/pattern suatu variable/ekspresi. Macro ini mengembalikan boolean `true` jika sesuai.
Contoh:
```rust
let num = 123;
let ret = matches!(num, 4);
println!("{}", ret);

let option = Some(5);
let ret = matches!(option, Some(5));
println!("{}", ret);
```

#### Associated Methods ####
Tipe Result<T,E> memiliki beberapa methods untuk meng-*handle* tipe error recoverable diantaranya `unwrap`.
Contoh:
```rust
fn fallible_function<'a>() -> Result<i32, &'a str> {
    Err("error ninu ninu") // jika gagal
}

let resp = fallible_function();

/// unwrap akan mengembalikan nilai jika `Result` berhasil(Ok(T))
/// akan panic jika hasil tidak `Ok(T)`
/// *NOTE: hindari penggunaan `unwrap()` karena kita tidak pernah tau hasil dari fallible operation tersebut. Kecuali jika kita yakin 100% bahwa hasil Ok(T).
let ret = resp.unwrap();

/// unwrap_err akan mengembalikan error jika `Result` error(Err(E))
/// akan panic jika hasil tidak `Err(E)`
/// *NOTE: hindari penggunaan `unwrap_err()` karena kita tidak pernah tau hasil dari fallible operation tersebut. Kecuali jika kita yakin 100% bahwa hasil Err(E).
let ret = resp.unwrap_err();

/// unwrap_or mengembalikan nilai jika Ok(T), 
/// jika tidak, maka akan mengembalikan nilai alternatif `123` sesuai dengan tipe data T.
/// Gunakan jika kita memiliki alternatif value selain dari yang diharapkan.
let ret = resp.unwrap_or(123);

/// unwrap_or_default mengembalikan nilai jika Ok(T), 
/// jika tidak, akan mengembalikan default value sesuai dengan tipe data T(e.g. i32 -> 0).
let ret = resp.unwrap_or_default();

/// unwrap_or_else akan mengembalikan nilai jika `Result` berhasil(Ok(T))
/// jika tidak, akan menjalankan suatu closure `FnOnce(E) -> T` dimana `E` adalah value dengan tipe yang sama dengan error `E` dari `Result<T,E>` dan `T` adalah value dengan tipe yang sama dengan hasil `T` dari `Result<T,E>` yang merupakan alternatif dari return value ketika yang sebelumnya error.
/// Gunakan ini ketika kita memiliki alternatif lain dari T dalam Result<T,E> dengan menjalankan suatu fungsi.
let ret = resp.unwrap_or_else(|x| -> i32 {
    println!("--:: {}", x);
    123
});
```

Masih banyak associated methods lainnya yang harus dieksplor sendiri yang memiliki tujuan masing-masing.

### Propagating Recoverable Error ###
Ketika kita ingin mengembalikan error ke fungsi `caller` dan seterusnya atau membuat rantai return error, maka kita bisa menggunakan operator `?` atau disebut juga operator `try`. Hal ini dilakukan ketika kita ingin menyerahkan error handling kepada fungsi yang memanggil. Syarat untuk try operator adalah memiliki tipe yang sama antara *caller* dan *callee*. 
*Try* operator berlaku untuk untuk tipe `Result<T,E>` dan `Option<T>`. 
Pada pembahasan error handling ini, kita menggunakan try operator untuk tipe `Result<T,E>`. Try operator pada `Result<T,E>` akan meng-*unwrap* `T` jika Ok, atau *return* fungsi jika error dimana tipe `Err(E)` antara *caller* dan *callee* compatible.
Contoh:
```rust
fn function1() -> Result<String, String> {
    let resp_funciton2 = function2()?; // if function2() return error, then this function will return the error from function2() into function1() error
    // do something with `resp`
    Ok(String::from("yay"))
}

fn function2<'a>() -> Result<i32, &'a str> {
    Err("error in function 2")
}
```
Pada contoh di atas, tipe data error dari fungsi2/method2 yang mem-propagasi error harus memiliki tipe yang sama. Pada contoh di atas terlihat kalau tipe data tidak sama, akan tetapi contoh di atas berjalan dengan benar. Hal ini karena selain tipe yang sama, tipe berbeda bisa dengan syarat masing-masing tipe meng-implementasi trait conversion pada Rust yaitu `From<T>`. Dengan implementasi trait konversi ini, tipe error tetap bisa di propagasi tanpa error. Hal ini karena tipe `String` implement method `from` dari trait `From<T>` sehingga bisa membaca error dari function2() berupa `&str`.


## Unrecoverable Error ##
Merupakan error yang tidak dapat ditolerir lagi karena bisa menyebabkan *undefined behaviour* pada program yang mana sebagian besar disebabkan oleh *logic error* dan juga beberapa side-effects. 
Pada saat panic terjadi, rust akan menghapus stack program atau yang disebut dengan `unwinding`.
Contoh dari error ini diantaranya: 
### index out of bound access ###
Di antara hal yang lumrah pada bahasa pemograman manapun. Yang membedakan adalah bahasa dengan *safety measure* akan langsung memberikan error ketika ada access diluar bound array. Bahasa lain seperti C mungkin akan memberikan *undefined behaviour* dengan memberikan nilai tidak valid bagi program. Rust memiliki bound checking pada saat pengaksesan array sehingga mengembalikan error ketika akses melebihi ukuran array.
Contoh:
```rust
let v = vec![1, 2, 3];

v[99];
```
### memanggil macro `panic!()` ###
Macro `panic!()` adalah macro untuk menyebabkan panic secara intentional ketika situasi tidak memungkinkan untuk meng-*recover* error yang terjadi.
```rust
fn this_fail() {
  panic!("im done");
}
```

### memanggil associated methods yang tidak sesuai dengan value yang diharapkan(e.g. `expect` atau `unwrap`)  ###
Kali ini kita akan membahas `expect` dimana kita ingin menambahkan pesan tambahan pada panic ketika terjadi error dari hasil `Result<T,E>` dan kita ingin panic. Method `expect` ini biasa digunakan untuk inisiasi program dengan berbagai pra-kondisi yang harus dipenuhi sebelum program berjalan seperti dependensi dan lainnya. Jika hal-hal itu tidak terpenuhi maka tentunya program tidak bisa berjalan dan harus exit dengan panic. `expect` digunakan ketika kita ingin menambahkan informasi tambahan terkait operasi yang kita lakukan yang tidak diketahui oleh compiler.
Contoh:
```rust
fn main() {
  let db_conn = init_db().expect("FAILED CONNECTING TO DB"); // will return value of `DB` from function, or panic error with additional information
}

fn init_db() -> Result<DB, String> {
  // logic for initiating DB connection
}
```

### testing assertions ###
Default behaviour dari fungsi-fungsi macro untuk test assertions ketika tidak sesuai ekspektasi adalah panic.
Contoh:
```rust
#[test]
fn test_addition() {
    let result = 2 + 2;
    assert_eq!(result, 5); // will cause fail tests and panic
}
```

Ketika kita ingin melakukan recovery terhadap situasi panic, maka kita dapat menggunakan fungsi `std::panic::catch_unwind` yang menerima closure yang mungkin terjadi panic.
Contoh:
```rust
let result = std::panic::catch_unwind(|| {
    panic!("im panic");
});
assert!(result.is_err()); // result akan mengembalikan error karena kondisi unwind terjadi di dalam closure karena fungsi `panic!()` dipanggil.
```
Pada development Rust, fungsi `catch_unwind` ini sangat jarang digunakan karena error yang disebabkan oleh panic memang error yang tidak bisa ditolerir yang bisa menyebabkan *undefined behaviour* pada program, ada side effects lainnya. Hal lain juga penyebab error panic ini jauh lebih kecil dibandingkan error yang disebabkan oleh IO atau side-effects lainnya, sehingga programmer sebaiknya mengantisipasi error ini pada saat development ditambah dengan testing.