## Instalasi ##

Rust memiliki tools installer untuk menginstal semua *toolschain* untuk memudahkan development Rust yaitu [rustup](https://rustup.rs/) dengan menjalankan perintah 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
*Untuk Windows bisa download file .exe pada link di atas.*

Pilih default installation, dan tunggu sampai installasi selesai. Di unix-based OS biasanya diinstall ke dalam 2 direktori yaitu `$HOME/.rustup` dan `$HOME/.cargo`. Direktori pertama tempat menyimpan master data program rustup. Sedangkan direktori ke dua tempat menyimpan toolschain yang dimanage oleh rustup itu sendiri. Biasanya rustup akan secara otomatis meng-ekspor env yang ada di `$HOME/.cargo/env` ke dalam env variable, misal `$HOME/.zshenv` atau `$HOME/.zshrc`. Path env yang dituju adalah `$HOME/.cargo/bin`, tempat executable dari toolschain rust bersemayam. Kalau path nya belum ada, tinggal diarahkan ke `$HOME/.cargo/bin`. Tools yang diinstall diantaranya:
- cargo : builder serta package management untuk Rust
- cargo-clippy dan clippy-driver: linting tools
- cargo-fmt dan rustfmt : formatting tools
- cargo-miri : middle level interpreter untuk Rust IR(Intermediate Representation)
- rls : rust language server
- rust-gdb : debugging tools menggunakan gdb
- rust-lldb : debugging tools menggunakan lldb
- rustc : compiler rust
- rustup : rustup itu sendiri
- rustdoc : doc tools

Ketika semua tools di atas sudah terinstall dan path environment sudah di setup, maka silahkan cek version, misal:
- `rustc --version` untuk versi bahasa rust yang digunakan.
- `cargo --version` untuk versi cargo yang digunakan.