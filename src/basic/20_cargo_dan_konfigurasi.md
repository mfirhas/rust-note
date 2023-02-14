# Cargo dan Konfigurasi #

Kita akan membahas mengenai build system dan package manager Rust yaitu **Cargo**, beserta konfigurasi projek Rust menggunakan standard konfig file Cargo yaitu .toml.
Kita sudah membahas mengenai installasi Rust yang sudah include Cargo menggunakan rustup. Pada chapter **toolchain** kita sudah membahas beberapa commands yang ada di Cargo untuk daily usages di dalam Rust. Kali ini kita akan membahas lebih lanjut dan juga konfigurasi projek Rust.

## Struktur Projek Cargo ##
Pada module 4 Struktur kita telah membahas struktur code base secara sederhana yaitu terdiri dari directory `src`, `target` dan file `Cargo.toml` dan `Cargo.lock`. 
Kali ini kita akan membahas kaitannya dengan Cargo dan konfigurasi nya. Berikut struktur codebase cargo secara keseluruhan:
```
.
├── Cargo.lock                          # hasil compiled Cargo.toml
├── Cargo.toml                          # konfigurasi projek cargo
├── src/                                # directory berisi codebase utama
│   ├── lib.rs                          # crate non-executable, to be imported
│   ├── main.rs                         # main crate for executable
│   └── bin/                            # folder konvensi cargo dimana cargo akan menganggap file2 .rs di dalam ini sebagai entry point executable
│       ├── named-executable.rs         # executable ketika di run/build `cargo run/build --bin named-executable
│       ├── another-executable.rs       # executable ketika di run/build `cargo run/build --bin another-executable
│       └── multi-file-executable/      # executable ketika di run/build `cargo run/build --bin multi-file-executable
│           ├── main.rs                 # ^command di atas akan meng-execute file main.rs ini
│           └── some_module.rs          # module yang akan dipanggil oleh main.rs di dalam directory ini.
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/                              # contain integration tests
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

## Konfigurasi Cargo.toml ##
Projek Rust menggunakan Cargo sebagai build system menggunakan Cargo.toml sebagai file konfigurasi projek. Komponen lengkap dari Cargo.toml bisa dilihat di [sini](https://doc.rust-lang.org/cargo/reference/manifest.html), akan tetapi pada kali ini kita cuma akan membahas yang umum pada projek yaitu:
- [package] : Setiap distributable rust program disebut package. Setiap package memiliki konfigurasi cargo di dalam Cargo.toml. Memiliki beberapa sub-konfigurasi diantaranya: *name*, *version*, *edition*, dan lainnya.
- [profile] : build profile sebagai konfigurasi kompilasi program. Terdiri dari *dev* dan *release*.
- Target tables: Mendeklarasi target kompilasi/build cargo. Memiliki jenis di antaranya: [lib], [[bin]], [[example]], [[test]], [[bench]].
- Dependency tables: Mendeklarasi dependencies dari package tersebut.
- [workspace] : konfigurasi Cargo.toml untuk level workspace.
- [features] : kondisional kompilasi

### [package] ###
Secara default ketika menginisiasi program, Cargo menginisiasi 3 fields name, version, dan edition. 2 field pertama minimal wajib untuk cargo dapat membaca sebuah package. *name* mengindikasikan nama package ketika akan di-distribusikan baik internal maupun eksternal. Versi menandakan versi dari package. Berikut beberapa fields lainnya:
- **name**: nama package yang digunakan sebagai identifier ketika mengekspor internal(memanggil sesama crates) atau eksternal(crates.io).
- **version**: versi dari package
- **authors**: daftar owners, maintainers, apapun yang pernah ngurus projek tersebut.
- **edition**: Rust edition. Cargo secara otomatis menggenerate sesuai dengan versi cargo yang digunakan untuk menggenerate.
- **rust-version**: Minimal rust version supported.
- **description**: deskripsi package
- **documentation**: link ke dokumentasi package.
- **repository**: link ke repo
- ...dan masih banyak lainnya silahkan di cek di sini [package](https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section)

Contoh:
```ini
[package]
name = "basics_cargo"
version = "0.1.0"
edition = "2021"
...
```


### Build Profile ###
Terdapat 2 jenis build profile di dalam Rust: **dev** dan **release** profile. 
- **dev** profile: merupakan profile build untuk level development dimana beberapa config di-setup default untuk development mode. Berikut konfigurasi beserta default values dari dev mode:
```ini
[profile.dev]
# optimization level.
# 0 = no optimization, used for development/debugging, faster compilation, slower binary
# 1 = basic optimizations, perform little bit optimization, but not as much as producing production binary.
# 2 = some optimizations, more than basics optimization, might be used in production if faster compilation needed than faster binary.
# 3 = full optimizations, apply all possible optimizations to the code resulting in slowest compilation, but fastest binary.
# "s" = optimization for binary size
# "z" = no loop vectorization
opt-level = 0

# untuk mengaktifkan debuginfo pada binary hasil build, bekerja sama dengan llvm untuk generate debuginfo-nya
debug = true

# debug info dipisah dari file hasil compiled. Platform specific.
split-debuginfo = '...'  # Platform-specific.

# runtime validation checking for dev mode, like `debug_assert!()` macro.
debug-assertions = true

# Memeriksa overflow pada operasi terhadap integer.
# true: panic jika overflow.
# false: circular effect jika overflow, misal 255_u8 + 1_u8 = 3.
overflow-checks = true

# link-time optimization merupakan fitur llvm dimana melakukan optimisasi cross module/code unit/crates.
# thin/false: melakukan sedikit optimisasi pada saat linking stage dalam proses kompilasi. Lebih cepat dalam proses kompilasi, tetapi tidak menghasilkan binary yang optimal.
# fat/true: melakukan optimisasi keseluruhan pada saat compile time menyebabkan kompilasi lebih lama, tetapi menghasilkan binary yang lebih optimal.
# off: tidak melakukan lto sama sekali
lto = false

# Apa yang akan dilakukan ketika panic terjadi:
# unwind: hapus stackframe(reset) lalu stop program
# abort: langsung stop program
panic = 'unwind'

# incremental build, menyimpan state2 untuk kompilasi berikutnya sehingga tidak harus build dari scratch(faster compilation)
# true: incremental
# false: clean build, used for production release
incremental = true

# codegen-units adalah serpihan hasil build suatu crate. Semakin banyak maka proses kompilasi semakin cepat karena compiler akan melakukan kompilasi parallel sebanyak mungkin, akan tetapi ini menghasilkan binary kurang optimal sehingga hanya cocok untuk dev mode. Untuk production release menggunakan angka yang lebih sedikit dari dev mode.
codegen-units = 256

# https://en.wikipedia.org/wiki/Rpath
rpath = false
```
- **release** profile: merupakan profile build untuk level production dimana beberapa config di-setup se-*optimized* mungkin untuk production release. Berikut konfigurasi beserta default values dari release mode:
```ini
[profile.release]
opt-level = 3
debug = false
split-debuginfo = '...'  # Platform-specific.
debug-assertions = false
overflow-checks = false
lto = false
panic = 'unwind'
incremental = false
codegen-units = 16
rpath = false
```

### Target ###
Merupakan hasil kompilasi kedalam binary/object file ketika projek di-build. Setiap target direpresentasikan oleh crate. Terdapat 5 jenis target di-antaranya:
- **Library(`[lib]`)**: hasil kompilasi dari crate library di dalam direktori src. 1 package hanya bisa memiliki 1 library target/crate.
- **Binaries(`[[bin]]`)**: hasil kompilasi dari crate binary di dalam direktori src. 1 package dapat memiliki beberapa target/crate.
- **Examples(`[[example]]`)**: hasil kompilasi dari crate example di dalam direktori sendiri dari root level. 1 examples bisa memiliki beberapa executable example.
- **Tests(`[[test]]`)**: hasil kompilasi dari crate test di dalam direktori sendiri dari root level. 1 tests bisa memiliki beberapa executable test.
- **Benchmarks(`[[bench]]`)**: hasil kompilasi dari crate benchmark di dalam direktori sendiri dari root level. 1 benchmarks bisa memiliki beberapa executable benchmark.

**Konfigurasi target**
```ini
[lib]/[[bin]]/[[example]]/[[test]]/[[bench]]
name = "foo"           # Nama target yang akan digunakan sebagai identifier pada saat memanggil/mengimport target.
path = "src/lib.rs"    # source file relative terhadap Cargo.toml
test = true            # Is tested by default.
doctest = true         # Documentation for lib
bench = true           # Is benchmarked by default.
doc = true             # Is documented by default.
plugin = false         # Used as a compiler plugin (deprecated).
proc-macro = false     # Set to `true` for a proc-macro library.
harness = true         # Use libtest harness.
edition = "2015"       # The edition of the target.
crate-type = ["lib"]   # Binaries, Tests, dan Benchmarks set to "bin", others can be "lib" and/or "proc-macro" for Libraries and Examples
required-features = [] # Features required to build this target (N/A for lib).
```

### Dependencies ###
Merupakan list dari semua dependencies yang digunakan oleh program/projek cargo tersebut. Terdapat 4 jenis dependencies:
- [dependencies] : dependencies yang akan dikompilasi bersama binaries hasil akhir executable dengan perintah "cargo build..."
- [dev-dependencies] : dependencies yang hanya di kompilasi pada saat melakukan testing dengan perintah "cargo test...", juga untuk examples and benchmarks.
- [build-dependencies] : dependencies untuk build script yang berjalan ketika menginisiasi program.
- [target] : dependencies specific untuk platform tertentu(e.g. OS tertentu)

**Spesifikasi dependencies**
TODO

### [workspace] ###
Deklarasi konfigurasi workspace yang memiliki beberapa packages di dalamnya. Terdapat dua jenis penulisan workspace:
- Workspace dengan root package: 
  - Terdapat satu package di level root workspace sebagai root package
  - Contoh deklarasi:
  ```ini
  [workspace]
  members = ["subpackage1","subpackage2"]

  # root package
  [package] 
  name = "hello_world" # the name of the package
  version = "0.1.0"    # the current version, obeying semver
  authors = ["Alice <a@example.com>", "Bob <b@example.com>"]
  ```
- Workspace tanpa root package
  - Semua packages berada pada level yang sama
  - Contoh deklarasi:
  ```ini
  [workspace]
  members = [
    "package1",
    "package2",
    ...]
  ```

#### [workspace.package] ####
Ketika ingin mendeklarasi package yang bisa di-inherited oleh members workspace.
Contoh:
```ini
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["bar"]

[workspace.package]
version = "1.2.3"
authors = ["Nice Folks"]
description = "A short description of my package"
documentation = "https://example.com/bar"
```
Ketika ingin inherit beberapa keys, bisa menggunakan `{key}.workspace = true`
```ini
# [PROJECT_DIR]/bar/Cargo.toml
[package]
name = "bar"
version.workspace = true
authors.workspace = true
description.workspace = true
documentation.workspace = true
```

#### [workspace.dependencies] ####
Ketika ingin mendeklarasi dependencies yang bisa di-inherited oleh members workspace.
Contoh:
```ini
# [PROJECT_DIR]/Cargo.toml
[workspace]
members = ["bar"]

[workspace.dependencies]
cc = "1.0.73"
rand = "0.8.5"
regex = { version = "1.6.0", default-features = false, features = ["std"] }
```
Ketika ingin inherit beberapa keys, bisa menggunakan `{key}.workspace = true`
```ini
# [PROJECT_DIR]/bar/Cargo.toml
[package]
name = "bar"
version = "0.2.0"

[dependencies]
regex = { workspace = true, features = ["unicode"] }

[build-dependencies]
cc.workspace = true

[dev-dependencies]
rand.workspace = true
```