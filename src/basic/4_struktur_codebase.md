# Struktur Codebase #

Kita akan mempelajari bagaimana struktur code, file dan direktori pada projek rust. Disini kita tidak membahas mengenai kaitannya dengan business domain atau business logic apapun, serta juga tidak membahas arsitektur-arsitektur projek tertentu seperti SOLID, Clean Code, Clean Arch, dan lainnya. Kita hanya membahas pada konteks *build system* di dalam projek rust menggunakan *cargo*. 

## Struktur Direktori ##
Konvensi direktori yang dihasilkan ketika inisiasi dan jalankan projek rust menggunakan cargo pertama kali adalah:
```
<project-dir>
  |--src/
      |--main.rs/lib.rs # file utama untuk binary(main) atau library(lib)
  |--target/ # direktori berisi hasil cargo build
  |--Cargo.lock
  |--Cargo.toml # konfigurasi package dan depedensi
```

## Jenis Program ##
Setiap program di dalam Rust merupakan hasil kompilasi dari crate-crate yang terdapat di dalam direktori *src/*.
Terdapat 2 jenis crate di dalam rust yaitu: 
- **binary** : Crate yang menghasilkan executable program. File utama memiliki deklarasi fungsi `fn main()` sebagai entry point program di dalam file `main.rs`.
- **library** : Crate yang menghasilkan non-executable program berupa pendukung/pelengkap/plugin untuk codebase lainnya. Tidak memiliki fungsi main hanya deklarasi-deklarasi module-module di dalam file `lib.rs`.

## Komponen Codebase ##
Di dalam codebase Rust terdapat beberapa jenis komponen-komponen yang keseluruhannya membentuk projek utuh yang ditulis dalam Rust.
Berikut komponen-komponen tersebut mulai dari yang terkecil:
- **Types**: Tipe data yang merupakan komponen terakhir yang bisa diakses dalam struktur codebase. Ada beberapa jenis tipe data di dalam Rust yaitu:
  |Types       | Desc.                                                        | Naming Convention    |
  |------------|--------------------------------------------------------------|----------------------|
  | **const**  | Values embeded at compile time                               | SCREAMING_SNAKE_CASE |
  | **static** | Values allocated once at compile time                        | SCREAMING_SNAKE_CASE |
  | **trait**  | Declaration of shared types, constants, and functions/methods| PascalCase           | 
  | **struct** | Product Type                                                 | PascalCase           | 
  | **enum**   | Sum Type                                                     | PascalCase           | 
  | **fn**     | Functions or Methods                                         | snake_case           | 
- Module(**mod**) : Module merupakan segment kode yang memiliki cakupan kecil dan besar hingga satu file. Komponen ini berguna untuk memisahkan segment program dan melakukan enkapsulasi struktur program. Untuk naming conventions menggunakan *snake_case*. Module memiliki 3 lokasi ketika program dibuild yaitu:
  - Inline: deklarasi mod langsung dengan kode menggunakan kurung kurawal.
  - Di dalam file dengan nama yang sama dengan mod(jika mod 1 level dengan file utama crate(`main.rs`/`lib.rs`)), atau di dalam direktori dengan nama yang sama dengan nama mod(untuk submodule)
  - Di dalam direktori dengan nama yang sama dengan mod di dalam file `mod.rs`.
- Crate : Representasi dari keseluruhan komponen-komponen di atas ke dalam *binary* crate atau *library* crate yang terletak di dalam direktori *src/*. Untuk binary ditentukan dengan fungsi `main` di dalam salah satu file di dalam crate itu yang berfungsi sebagai entry point eksekusi program. Untuk naming convention file menggunakan *snake_case*.
- Package : Package membungkus semua crate-crate yang telah kita buat di atas. Di level ini juga kita meletakkan file konfigurasi depedensi berupa `Cargo.toml` untuk mendeklarasikan crate-crate yang akan digunakan di dalam package tersebut. Package juga merupakan cara untuk men-distribusikan code kita untuk di reuse di tempat lain baik sebagai 3rd party atau internal project/service.
- Workspace : Merupakan gabungan dari beberapa package-package dengan `Cargo.toml` yang berisi deklarasi package-package terkandung. Ketika kita menjalankan perintah cargo di dalam workspace, maka akan ditelusuri melalui package-package yang ada di dalam konfigurasi toml workspace.

## Visibiliti Komponen ##
Setiap komponen di atas bersifat private dan hanya bisa diakses menggunakan keyword **pub** di depan setiap deklarasi komponen. 
Contoh:
```
pub mod module {
    pub const CONSTANT: &str = "this is constant";
}
```
Maka `CONSTANT` dapat diakses lewat `...::module::CONSTANT;`.

## Komposisi Komponen ##
Berikut peraturan-peraturan tentang komposisi komponen-komponen di dalam codebase:
- 1 workspace berisi minimal 1 package
- 1 package hanya dapat berisi:
  - 1 binary crate, atau
  - 1 library crate, atau
  - 1 binary crate dan 1 library crate, atau
  - N binary crates dan 1 library crate
  - N sub-packages
- 1 crate merupakan 1 dari 2 jenis program di dalam Rust yaitu *binary*(executable) dan *library*(non-executable)
- 1 module bisa berisi module lain atau berbagai macam komponen lebih kecil lainnya.

Contoh dari komposisi di atas bisa dilihat di [rust-namespace](https://github.com/mfathirirhas/rust-namespace)

## Contoh Struktur Codebase ##
### 1 Binary Crate ###
```
sample_1_binary/
|--src/
    |--file1/                   # direktori submodule yang diakses dari file1.rs
        |--sub_file_1/          # direktori submodule yang diakses dari sub_file_1.rs
            |--sub_sub_file_1/  # nama submodule yang diakses dari sub_file_1.rs
                |--mod.rs       # kode submodule sub_sub_file_1 yang diakses dari sub_file_1.rs
        |--sub_file_1_2.rs      # module sub_file_1_2.rs diakses dari file1.rs
        |--sub_file_1.rs        # module sub_file_1.rs diakses dari file1.rs
    |--file2/                   # direktori module file2 yang diakses dari main.rs
        |--mod.rs               # module file2 yang diakses dari main.rs
        |--sub_mod.rs           # submodule sub_mod yang diakses dari file2/mod.rs
    |--file1.rs                 # module `file1` diakses di main.rs
    |--main.rs                  # tempat entrypoint program fn main()
|--target/
|--Cargo.lock
|--Cargo.toml
```

### 1 Library Crate ###
```
sample_1_binary/
|--src/
    |--file1/                   # direktori submodule yang diakses dari file1.rs
        |--sub_file_1/          # direktori submodule yang diakses dari sub_file_1.rs
            |--sub_sub_file_1/  # nama submodule yang diakses dari sub_file_1.rs
                |--mod.rs       # kode submodule sub_sub_file_1 yang diakses dari sub_file_1.rs
        |--sub_file_1_2.rs      # module sub_file_1_2.rs diakses dari file1.rs
        |--sub_file_1.rs        # module sub_file_1.rs diakses dari file1.rs
    |--file2/                   # direktori module file2 yang diakses dari main.rs
        |--mod.rs               # module file2 yang diakses dari main.rs
        |--sub_mod.rs           # submodule sub_mod yang diakses dari file2/mod.rs
    |--file1.rs                 # module `file1` diakses di main.rs
    |--lib.rs                   # tempat mendeklarasi dan meng-eksport semua module2
|--target/
|--Cargo.lock
|--Cargo.toml
```
Sama dengan direktori 1 Binary di atas, bedanya file utama `lib.rs` cuma berisi dekalrasi-deklarasi module dan tanpa fungsi main di dalamnya.

### 1 Binary & 1 Library Crate ###
```
sample_1_binary_1_library/
|--src/
    |--binary/
        |--main.rs
    |--library/
        |--funcs.rs
        |--lib.rs
|--target/
|--Cargo.lock
|--Cargo.toml
```
Pada direktori di atas, terdapat 2 crate dengan file utama masing-masing, binary crate(main.rs) dan library crate(lib.rs). Library crate tidak berisi executable program, hanya sekumpulan kode yang mendukung development yang terjadi di binary dan dipanggil ke dalam binary crate.

### N Binary & 1 Library Crate ###
```
sample_n_binary_1_library/
|--src/
    |--bin/
        |--main_4.rs
        |--main_5.rs
    |--lib/
        |--lib.rs
    |--main_3/
        |--main_3.rs
    |--main_1.rs
    |--main_2.rs
|--target/
|--Cargo.lock
|--Cargo.toml
```
Terdapat 5 binary crate dengan nama file utama *main_1.rs*, *main_2.rs*, *main_3.rs*, *main_4.rs*, *main_5.rs*, dan 1 library crate dengan nama *lib.rs* di dalam direktori *lib/*.

### N Sub-packages ###
```
sample_n_sub-packages/
|--src/
    |--src/
    |--package-1/
    |--package-2/
    |--...
|--target/
|--Cargo.lock
|--Cargo.toml
```
Package `package-1` dan `package-2` merupakan package tersendiri dengan struktur yang sama seperti diluar. `package-1` dan `package-2` akan diimport oleh package di luar. Deklarasi import sub-packages ke dalam package luar di toml adalah:
```toml
package-name = {path = "path-to-subpackage", package = "package-name"}
```
- **package-name**: nama yang akan di import menggunakan `use package-name::`
- **path-to-subpackage**: path ke subpackage dimulai dari root package. Pada contoh di atas maka bisa digunakan: `package-1`
- **package-name**: nama package dari path di atas. Biasanya nama package sama dengan nama directory.

Secara *convetion* cargo, cargo akan secara otomatis membaca binary crates yang ada di dalam direktori *src/bin/<file_name.rs>* tanpa mendeklarasikan crate path di dalam *Cargo.toml*. Jika kita tidak mengikuti *convention* tersebut, maka kita harus deklarasikan nama dan path dari crate di dalam *Cargo.toml* di bawah section `[package]` di atas section `[dependencies]` seperti contoh:
```ini
...

[[bin]]
name = "main_1"
path = "src/main_1.rs"

[lib]
name = "lib"
path = "src/lib/lib.rs"

[[bin]]
name = "main_2"
path = "src/main_2.rs"

[[bin]]
name = "main_3"
path = "src/main_3/main_3.rs"

...
```

### Workspace ###
```
sample_workspace/
|--package_1_binary/
    |--src/
        |--file1/                   # direktori submodule yang diakses dari file1.rs
            |--sub_file_1/          # direktori submodule yang diakses dari sub_file_1.rs
                |--sub_sub_file_1/  # nama submodule yang diakses dari sub_file_1.rs
                    |--mod.rs       # kode submodule sub_sub_file_1 yang diakses dari sub_file_1.rs
            |--sub_file_1_2.rs      # module sub_file_1_2.rs diakses dari file1.rs
            |--sub_file_1.rs        # module sub_file_1.rs diakses dari file1.rs
        |--file2/                   # direktori module file2 yang diakses dari main.rs
            |--mod.rs               # module file2 yang diakses dari main.rs
            |--sub_mod.rs           # submodule sub_mod yang diakses dari file2/mod.rs
        |--file1.rs                 # module `file1` diakses di main.rs
        |--main.rs                  # tempat entrypoint program fn main()
    |--Cargo.toml
|--package_2_binary/
    |--src/
        |--file1/                   # direktori submodule yang diakses dari file1.rs
            |--sub_file_1/          # direktori submodule yang diakses dari sub_file_1.rs
                |--sub_sub_file_1/  # nama submodule yang diakses dari sub_file_1.rs
                    |--mod.rs       # kode submodule sub_sub_file_1 yang diakses dari sub_file_1.rs
            |--sub_file_1_2.rs      # module sub_file_1_2.rs diakses dari file1.rs
            |--sub_file_1.rs        # module sub_file_1.rs diakses dari file1.rs
        |--file2/                   # direktori module file2 yang diakses dari main.rs
            |--mod.rs               # module file2 yang diakses dari main.rs
            |--sub_mod.rs           # submodule sub_mod yang diakses dari file2/mod.rs
        |--file1.rs                 # module `file1` diakses di main.rs
        |--main.rs                  # tempat entrypoint program fn main()
    |--Cargo.toml
|--package_3_library/
    |--src/
        |--file1/                   # direktori submodule yang diakses dari file1.rs
            |--sub_file_1/          # direktori submodule yang diakses dari sub_file_1.rs
                |--sub_sub_file_1/  # nama submodule yang diakses dari sub_file_1.rs
                    |--mod.rs       # kode submodule sub_sub_file_1 yang diakses dari sub_file_1.rs
            |--sub_file_1_2.rs      # module sub_file_1_2.rs diakses dari file1.rs
            |--sub_file_1.rs        # module sub_file_1.rs diakses dari file1.rs
        |--file2/                   # direktori module file2 yang diakses dari main.rs
            |--mod.rs               # module file2 yang diakses dari main.rs
            |--sub_mod.rs           # submodule sub_mod yang diakses dari file2/mod.rs
        |--file1.rs                 # module `file1` diakses di main.rs
        |--lib.rs                  # tempat entrypoint program fn main()
    |--Cargo.toml
|--target/
|--Cargo.lock
|--Cargo.toml
```

Workspace punya toml sendiri yang berisi deklarasi member2 package di dalamnya:
contoh:
```ini
[workspace]

members = [
    "package_1_binary",
    "package_2_binary",
    "package_3_library",
]
```