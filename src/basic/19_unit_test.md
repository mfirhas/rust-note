# Unit Test #

Unit Test adalah untuk mengvalidasi/mengverifikasi suatu fungsi/method terhadap semua kemungkinan input dan output. Unit test harus mengcover banyak edge cases dari suatu program agar mengurangi bugs. Unit Test juga digunakan dalam *code coverage* agar semua logic terkover dari semua path yang mungkin ada pada program. Ada berbagai macam cara Unit Test tergantung bahasa pemograman yang digunakan. Sebagian besar Unit Test bersifat idempotent sehingga ketika dijalankan berulang-ulang tidak ada yang berubah dan akan selalu menghasilkan output yang sama, dalam bahasa lain bisa juga disebut *stateless*. Hal ini bagian dari otomatisasi testing yang juga akan dilakukan dalam pipeline CI/CD. Kali ini kita akan bahas mekanisme testing dalam Rust.

Rust menggunakan 2 macro untuk menandai segment code untuk testing, yaitu `#[cfg(test)]` dan `#[test]`.
```rust
// #[cfg(test)] menandakan module sebagai testing sehingga tidak dimasukkan pada saat kompilasi/build(cargo build) program, hanya dieksekusi dengan perintah `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    // #[test] menandakan fungsi testing yang akan dieksekusi oleh `cargo test`.
    #[test]
    fn test_add() {
        assert!(5 == add(3, 2));  // memvalidasi suatu ekspresi bernilai true
        assert_eq!(5, add(3, 2)); // memvalidasi 2 values/ekspresi memiliki nilai yang sama
        assert_ne!(2, add(3, 2)); // memvalidasi 2 values/ekspresi tidak memiliki nilai yang sama

        // secara manual panic untuk menandakan failed test
        if add(3, 2) != 5 {
            panic!("unexpected");
        }
    }
}
```

`#[cfg(test)]` membuat semua code yang berada dibawah macro ini tidak dimasukkan pada saat kompilasi ke dalam binary dan hanya berjalan pada perintah `cargo test`.
Semua fungsi test harus memiliki macro `#[test]` untuk menandakan fungsi yang akan dijalankan. Semua fungsi-fungsi ini berjalan secara parallel secara default.
Jika ingin berjalan tidak paralel, kita cukup membatasi thread eksekusi testing menjadi 1
```
$ cargo test -- --test-threads=1
```

`#[test]` hanya berlaku untuk fungsi/method yang *synchronous*, untuk fungsi/method yang bersifat *asynchronous* bisa menggunakan library async seperti tokio menggunakan macro `#[tokio::test]`.

## Organisasi Unit Testing ##
Kode unit test sebaiknya dikelompokkan ke dalam module terpisah sehingga lebih mudah menandakan segment code yang digunakan untuk testing. Ketika unit testing code semakin bertambah maka akan memudahkan memberikan configurasi `#[cfg(...)]` pada segment code di bawah `mod` tersebut.

Tidak ada konvensi khusus mengenai dimana letak kode unit tests ini. Code unit testing sama layaknya code lainnya di dalam rust yaitu berlaku enkapsulasi public dan private dari suatu kode. Sehingga ada pros and cons terhadap beberapa implementasi unit testing.
- Jika unit tests di taruh di file terpisah seperti pada konvensi Golang, seperti *file_name_test.rs*, maka code yang bisa diimport ke dalam unit tests hanyalah kode public. Hal ini karena sebuah file di dalam rust juga secara tidak langsung merupakan module `mod` tersendiri, dan antar sesama file merupakan sibling yang tidak bisa mengakses private functions mod/file lain dalam level yang sama(sibling). Sisi positif nya adalah separasi kode production dan kode testing per file.
Contoh:
```rust
/// a.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn priv_add(a: i32, b: i32) -> i32 {
    a + b
}

// ------------------------------------

/// a_test.rs
/// 
/// import module dari dari file lain lewat namespace crate karena bukan di dalam file main.rs/lib.rs, dan merupakan module/file yang berada pada level yang sama. 
use crate::a;

#[test]
fn test_add_file() {
    // akses a::add berhasil karena fungsi add() public
    assert!(5 == a::add(3, 2));
    assert_eq!(5, a::add(3, 2));
    assert_ne!(2, a::add(3, 2));

    if a::add(3, 2) != 5 {
        panic!("unexpected");
    }

    // akses a::priv_add tidak bisa karena fungsi priv_add private
    if a::priv_add(3, 2) != 5 {
        panic!("unexpected");
    }
}
```

- Jika unit tests di taruh di dalam module di dalam suatu file/module, maka relasi kedua module ini menjadi parent-child, dimana kode child bisa mengakses semua code yang ada di parent level. Cara ini digunakan ketika ada banyak private code di dalam suatu module sehingga butuh deklarasi unit tests di dalam submodule tersebut. Sisi buruk nya tentu kode production bercampur dengan kode testing, dalam sudut pandang user tentunya, bukan compiler.
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn priv_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::{add, priv_add};

    // kedua fungsi add dan priv_add dapat diakses dari dalam submodule karena relasi module ini(tests) dan parent nya lewat `use super::*`.

    #[test]
    fn test_add() {
        assert_eq!(5, add(3, 2));
    }

    #[test]
    fn test_priv_add() {
        assert_eq!(5, priv_add(2, 3));
    }
}
```

## Testing Kode yang mungkin Panic ##
Ada kalanya kita akan menemukan suatu operasi yang memiliki kemungkinan panic. Secara default panic adalah pertanda failed tests di dalam rust, sehingga untuk meng-*assert* panic ini dibutuhkan macro `#[should_panic]` setelah macro `#[test]`.
```rust
fn im_panic() {
    panic!("OH NO!!!");
}

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_im_panic() {
        im_panic();
    }
}
```
Sekalipun ketika test dijalankan menghasilkan stack trace panic, kode test ini dianggap berhasil oleh `cargo test`.
Jika kode berhasil justru dianggap test gagal karena macro `#[should_panic]`.

## Menggunakan Enum Result<(),E> ##
Selain panic, kita bisa menggunakan return value `Result<(),E>` pada saat unit testing.
```rust
#[test]
fn test_add_result() -> Result<(),String> {
    let expected = 5;
    let result = add(3,2);
    if result != expected {
        return Err("ERROR!!!".to_owned());
    }
    Ok(())
}
```
Tipe `T` haruslah `()` karena merupakan konvensi dari cargo. Untuk tipe `E` kita bisa menggunakan tipe apapun.

## Menampilkan output ##
Secara default jika kita mem-print sesuatu menggunakan `println!` atau `dbg!` ketika test berhasil menggunakan `cargo test` maka tidak memprint apa-apa karena ketika test berhasil semua output ke stdout di-*capture* oleh rust lebih duluan. Jika kita ingin tetap menampilkan output tersebut bisa menggunakan flag `cargo test -- --show-output`.

## Menfilter Unit Tests ##
Ketika kita hanya ingin menjalankan unit test tertentu saja kita bisa menulis `cargo test <pathname/filename/part_of_path_or_file>`, dimana argumen tambahan ke dalam command tersebut merupakan pattern yang menyerupai fullpath(mod), filename, atau nama fungsi test.
Contoh:
```bash
$ cargo test test_add # menjalankan fungsi test test_add atau pathname/filename yang menyerupai
# contoh file-file yang mungkin dijalankan oleh command di atas adalah:
# test_add
# test_add_success
# test_add_failed
# test_a

$ cargo test test_ # menjalankan fungsi a::b::c::test_
# file-file atau kode test yang akan di jalankan dengan perintah di atas adalah semua kode tests dengan path `a::b::c::test_*`
```

## Mengabaikan Unit Tests ##
Mengabaikan fungsi test bisa menggunakan macro `#[ignore]` setelah deklarasi `#[test]`.
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn priv_add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::{add, priv_add};

    // kedua fungsi add dan priv_add dapat diakses dari dalam submodule karena relasi module ini(tests) dan parent nya lewat `use super::*`.

    #[test]
    fn test_add() {
        assert_eq!(5, add(3, 2));
    }

    #[test]
    #[ignore]
    fn test_priv_add() { // unit test will be ignored when we run cargo test
        assert_eq!(5, priv_add(2, 3));
    }
}
```