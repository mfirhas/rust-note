# Primitive Types #

Rust memiliki tipe-tipe paling dasar untuk menghandle beberapa data diantaranya numerik, karakter, kalimat, bytes, dan lainnya.

Berikut beberapa tipe-tipe dasar dalam Rust:

## Unit ##
Tipe data yang tidak berarti apa-apa. Biasanya digunakan ketika ingin meng-*ignore* deklarasi tipe data. 
Salah satu kegunaan tipe ini adalah untuk meng-*ignore* deklarasi tipe data untuk generic type.
Contoh:
```rust
()

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