# Toolchain #

Rust memiliki beberapa toolchain untuk berinteraksi dengan program rust yang digunakan untuk berbagai macam keperluan seperti kompilasi, build, formatting, linting, dan testing. Semua fungsionalitas ini ada di dalam program `cargo` yang telah kita install sebelumnya.

Untuk menjalankan program Rust, terdapat dua tool utama yang digunakan yaitu `rustc` yang merupakan compiler rust itu sendiri, dan `cargo` package management serta build tool untuk rust. Jika kita ingin mengkompilasi sebuah file saja, atau ingin memiliki opsi build untuk program rust yang ditulis, maka bisa menggunakan `rustc`. Tool kedua `cargo` lebih direkomendasikan untuk mem-*build* program yang sudah lebih dari 1 file serta memiliki depedensi satu sama lain termasuk dari 3rd party library. Cargo juga sudah mengabstraksikan banyak hal yang dibutuhkan untuk membangun artifak program dari banyak file dan depedensi dengan perintah yang lebih sederhana.

## rustc ##
Contoh penggunaan `rustc`:
Tulis contoh `Hello, World!` sederhana berikut dan compile dengan perintah `rustc <nama-file>`, maka akan menghasilkan binary output yang langsung dapat dieksekusi dengan menjalankan `./<nama-binary>`(asumsi menggunakan unix-based OS).
```rust
fn main() {
    println!("Hello, World!");
}
```

## cargo ##
Cargo lebih digunakan untuk projek dengan codebase yang multi depedensi satu sama lain serta juga dengan library luar. Cargo juga menyediakan banyak perintah untuk manage codebase dengan lebih mudah termasuk meng-inisialisasi codebase.

### Codebase baru ###
Ketika ingin memulai projek rust, dimulai dengan perintah `cargo init` atau `cargo new`. Kedua perintah ini terlihat mirip, bedanya `init` akan menciptakan source pada current directory, sedangkan `new` menciptakan source baru pada path dengan nama yang ditentukan.
Contoh:
`cargo new $HOME/code/rust/test-cargo --bin --vcs none`
Perintah di atas akan membuat projek rust baru pada path yang ditentukan dengan nama projek. Flag `--bin` digunakan untuk memberi tau cargo kita akan membuat projek yang akan menghasilkan executable binary. Alternatif lain adalah `--lib` untuk projek non-executable yang digunakan sebagai library. Kita akan bahas hal-hal ini pada pembahasan selanjutnya tentang struktur projek pada rust. Untuk `--vcs none` menginisiasi projek tanpa git, karena secara default cargo new akan menginisiasi projek dengan git.

### Build ###
Untuk membangun program serta menghasilkan runnable artifak maka kita bisa menjalankan perintah:
`cargo build` pada direktori projek. Default `build` adalah mode *debug* atau mode ketika dalam proses development. Ketika kode yang kita buat sudah pantas untuk naik ke production, maka ditambahkan flag release menjadi `cargo build --release`. Flag tambahan ini melakukan optimisasi program pada compile time sehingga artifak yang dihasilkan berjalan dengan optimal dan maksimal di production.

### Run ###
Ketika development tentunya kita tidak ingin setiap menjalankan program harus build ulang dan menghasilkan artifak lagi untuk sekedar testing. Oleh sebab itu kita bisa melakukan mode run dengan command `cargo run` pada direktori projek. Perintah ini mem-*build* serta menjalankan program secara langsung.

### Test ###
Perintah `cargo test` digunakan untuk menjalankan semua bagian program yang ditandai sebagai `test`. Kita akan bahas ini pada pembahasan lebih lanjut.

### Clippy ###
Perintah `cargo clippy` digunakan untuk menjalankan linter terhadap codebase kita untuk melakukan pengecekkan terhadap hal-hal yang berkaitan dengan style, convention, code organization, dan lainnya. Ada banyak opsi linting pada rust yang bisa digunakan atau kita buat sendiri. Tersedia banyak tools linting pada [clippy](https://rust-lang.github.io/rust-clippy/master/) dengan berbagai kategori.

### Formatting ###
Perintah `cargo fmt` akan meng-format seluruh code di semua file-file dengan ekstensi `.rs` menyesuaikan dengan format standar pada rust yang tentunya untuk tampilah kode yang lebih nyaman dibaca.

Masih banyak hal lain yang tersedia pada `cargo` beserta flag-flagnya yang harus dieksplorasi sendiri menyesuaikan kebutuhan.