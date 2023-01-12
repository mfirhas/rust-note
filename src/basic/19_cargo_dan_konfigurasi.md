# Cargo dan Konfigurasi #

Kita akan membahas mengenai build system dan package manager Rust yaitu **Cargo**, beserta konfigurasi projek Rust menggunakan standard konfig file Cargo yaitu .toml.
Kita sudah membahas mengenai installasi Rust yang sudah include Cargo menggunakan rustup. Pada chapter **toolchain** kita sudah membahas beberapa commands yang ada di Cargo untuk daily usages di dalam Rust. Kali ini kita akan membahas lebih lanjut dan juga konfigurasi projek Rust.

## Build Profile ##
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
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs

```

// berlanjut...