# String Types #

Rust memiliki 2 jenis tipe data dasar untuk berhubungan dengan *string*. Yaitu:

## Reference string (&str) ##
Reference string yaitu value string yang tidak memiliki *owner*, dalam artian hanya bisa di-pass sebagai reference dan *immutable*. 
String semacam ini bisa dibentuk dengan berbagai cara, diantaranya secara *hardcode* di kodingan, di-*referenced* dari data lainnya atau dibentuk dari array u8(utf8). Ketika string jenis ini didapat dari reference owned value, maka reference nya akan valid selama object owner nya valid(tidak out of scope).

Berikut penjelasan beberapa cara string jenis ini muncul:
- *Hardcode*: cara ini yaitu dengan menuliskan value string secara literal pada saat menulis kode program. String jenis ini bersifat static, yaitu di-*compile* bersama program ke dalam binary dan exist selama program berjalan.
Contoh: 
```rust
let s: &str = "this is literal string";
println!("{}",s);
```
String jenis ini diketahui ukuran nya pada saat compile time dan *immutable*. Ketika ingin memanipulasi string ini harus di-*owned* terlebih dulu(dijelaskan pada pembahasan selanjutnya).

- *Referenced*: cara ini yaitu dengan mengambil reference dari object(owned) **String** di runtime. 
Contoh:
```rust
let s: String = "this is owned string".to_string();
let ref_s: &str = s.as_str();
```
`ref_s` valid selama `s` tidak di `drop` atau keluar scope. Hal ini karena `s.as_str()` hanya memberika reference kepada value `&str` yang ada pada object `s` sehingga pemilik &str nya adalah `s`. String jenis ini dialokasikan pada *heap memory* karena `String` dialokasikan di *heap memory*.

- *Array u8(utf8)*: cara ini dibentuk ketika string di encode kedalam utf8, sehingga value array u8 bisa digunakan untuk membentuk string tsb.
Contoh:
```rust
let utf8 = [105, 109, 32, 102, 114, 111, 109, 32, 98, 121, 116, 101, 115];
let s_ref: &str = std::str::from_utf8(&utf8).unwrap();
println!("{}",s_ref); // print "this is owned string"
```
String jenis ini dialokasikan di dalam *stack memory*. 
String jenis ini lebih dianjurkan digunakan ketika kita memiliki string yang hanya *read-only*.


## Owned String (String) ##
String jenis ini merupakan string yang dibentuk pada saat runtime dimana ukurannya tidak diketahui dan bisa berubah-ubah sehingga perlu dialokasikan pada *heap memory*. String ini sifatnya di-*owned*, dalam artian memiliki owner yang mana value nya valid selama owner nya tidak out of scope dan tidak di-*drop*.
Contoh:
```rust
let owned_string: String = String::from("im owned");
println!("{}",owned_string);
```
String di atas di-*owned* oleh variable `owned_string`, selama `owned_string` masih di dalam scope yang sama, maka string valid digunakan. `owned_string` menjadi tidak valid ketika berpindah kepemilikan seperti di-*pass* ke dalam suatu fungsi atau berpindah scope `{}`.
String jenis ini digunakan ketika kita ingin lebih leluasa mengoperasikan suatu string.