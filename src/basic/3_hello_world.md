# Hello World #

Kita akan membuat program pertama dengan menggunakan `cargo` untuk memudahkan me-*manage* dan scale projek. 

Masukkan perintah berikut:
```
cargo new <path-to-your-new-project> --bin --vcs none
```

Direktori projek akan terbentuk pada path yang ditentukan dengan struktur direktori dan file-file yang konvensional dalam development rust. 
Karena kita membuat projek baru untuk executable maka kita menggunakan flag `--bin` dan akan menghasilkan default program baru berupa `Hello, world!` di dalam file `src/main.rs`.
```rust
fn main() {
    println!("Hello, World!");
}
```

Jalankan perintah:
```
cargo run
```
Maka akan muncul 
```
Hello, world!
```