# Standard Traits #

Berikut kita akan membahas beberapa trait standard yang biasa digunakan dalam development Rust. 

## Auto dan Marker Traits ##
Trait yang sudah secara otomatis di-implementasikan oleh compiler untuk hampir semua tipe data di Rust. 
Terdapat beberapa auto traits:

### Send ###
[Send](https://doc.rust-lang.org/std/marker/trait.Send.html): Trait yang menandakan tipe data yang *safe* untuk dikirim antar threads. Hampir semua tipe data pada Rust sudah implement `Send` secara automatis, pengecualian: `Rc<T>` karena smart pointer untuk borrow di dalam thread yang sama, versi *Send* dari Rc adalah Arc.

### Sync ###
[Sync](https://doc.rust-lang.org/std/marker/trait.Sync.html): Trait yang menandakan tipe data yang *safe* untuk mengirim reference dari suatu data antar threads. Pengecualian adalah tipe data yang memiliki pattern interior mutability seperti `Cell`, `RefCell`, dll.

### Sized ###
Tipe dengan trait Sized memiliki size yang diketahui pada saat compile-time sehingga bisa ditaruh di stack.
```rust
fn function<T: Sized>(t: T) {
    ...
}
```

Tipe yang tidak memiliki known sized at compile time:
```rust
let arr: [i32] = [2, 3, 4, 5];
// [i32] has no known size at compile time.
// use [i32; N] where N is fixed size, or
// use &[i32] where & indicates it's a reference pointer which size is pointer's size and data referenced from other defined places.
let s: S<[i32]> = arr;
```

Optional Sized:
```rust
fn function<T: ?Sized>(t: T) {
    ...
}
```
Tanda tanya didepan `Sized` menandakan bahwa bisa menaruh *Dynamically Sized Type* juga(tidak strict sized).

### Unpin ###
*Advance*

### UnwindSafe ###
*Advance*

### RefUnwindSafe ###
*Advance*