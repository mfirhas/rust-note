# Primitive Types #

Rust memiliki tipe-tipe paling dasar untuk menghandle beberapa data diantaranya numerik, karakter, kalimat, bytes, dan lainnya.

Berikut beberapa tipe-tipe dasar dalam Rust:

## Unit `()` ##
Tipe data yang tidak berarti apa-apa. Biasanya digunakan ketika ingin meng-*ignore* deklarasi tipe data. 
Salah satu kegunaan tipe ini adalah untuk meng-*ignore* deklarasi tipe data untuk generic type.
Contoh:
```rust
fn return_nothing() -> () {
    println!("do nothing");
}
```

## Boolean ##
| Type     | Description |
| -------- | ------------|
| true     | true        |
| false    | false       |

Contoh:
```rust
let b: bool = true;
```

## Integer ##
| Type        | Description                     |
| ----------- | ------------------------------- |                    
|  i8         |  8bit integer                   |
|  i16        |  16bit integer                  |
|  i32        |  32bit integer                  | 
|  i64        |  64bit integer                  |
|  i128       |  128bit integer                 |
|  isize      |  bit size depends on target arch|

Untuk tipe *unsigned* tinggal mengganti *i* menjadi *u* seperti u8, u16, u32, u64, u128, dan usize.
Contoh:
```rust
let n: i64 = 123345;
```

## Floating Point ##
| Type      | Description           |
| --------- | ----------------------|
| f32       |  32bit floating point |
| f64       |  64bit floating point |

Contoh:
```rust
let f: f64 = 123.5;
```

## Character ##
Menggunakan *single quote* seperti `'a'`.
Contoh:
```rust
let c: char = 'a';
```

Semua tipe data di atas harus diketahui ukurannya pada saat compile time dan tetap dalam ukuran.

## Pointer ##
Ada 2 jenis pointer di rust, yaitu reference pointer dan raw pointer.
- Reference Pointer(`&` & `&mut`): menggunakan ampersand `&`. Memiliki safety guaranty dari Rust menggunakan borrow checker. Reference pointer lebih sering digunakan karena safe dan dijamin oleh borrow checker rust. Tidak memerlukan unsafe operation dan value selalu valid.
- Raw Pointer(`*const` & `*mut`): menggunakan asterisk `*`. Tidak memiliki safety guaranty dari Rust menggunakan borrow checker. Deferencing raw pointer is unsafe.
Raw pointer jarang digunakan, biasanya untuk interop dengan FFI.

Contoh: 
```rust
let a = 123;
let p: *const i32 = &a;
assert!(!p.is_null());
let mut b = 123;
let p_mut: *mut i32 = &mut b;
assert!(!p_mut.is_null());
```