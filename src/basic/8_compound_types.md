# Compound Types #

Compound types merupakan tipe data yang terdiri dari beberapa tipe data yang dimasukkan ke dalam suatu variable. Tipe data ini bisa berisi tipe primitif, string, struct, tuple, enum atau lainnya.

## Array `[T;size]` ##
Array merupakan tipe data gabungan dari value-value dengan tipe yang sama. Size array bersifat *fixed* dengan value-value yang diinisialisasi pada saat compile time. Ownership dari array tergantung dari elemen-elemen yang terkandung. Jika mengandung tipe data yang *copyable* atau *reference* maka tidak ada ownership tertentu sehingga array tetap valid jika keluar dari scope. Sebaliknya jika elemen terkandung merupakan *heap-allocated* data, maka array tidak akan valid jika telah keluar dari scope.
Contoh:
```rust
let array_1 = [1,2,3,4,5];
println!("{}", array_1[0]); // print 1
println!("{}", array_1[1]); // print 2
println!("{}", array_1[2]); // print 3

let array_2: [i32; 5] = [1,2,3,4,5];
println!("{}", array_2[0]); // print 1
println!("{}", array_2[1]); // print 2
println!("{}", array_2[2]); // print 3
```

## Slice `&[T]` ##
Berbeda dengan array, slice memiliki size yang dinamis dan tidak dideklarasi pada saat *compile-time*. Slice merupakan tipe data reference yang mereferensikan ke suatu lokasi memori yang mengandung data di dalam slice. Slice bertujuan ketika kita ingin mendefenisikan array dengan jumlah data yang tidak pasti dan bisa berubah ke depannya. Berbeda dengan array yang ownership data tergantung elemen terkandung, Slice merupakan reference type sehingga slice tetap valid di scope manapun.
Contoh:
```rust
let slice_1 = &[1,2,3,4,5];
println!("{}", slice_1[0]); // print 1
println!("{}", slice_1[1]); // print 2
println!("{}", slice_1[2]); // print 3

let slice_2: &[i32] = &[1,2,3,4,5];
println!("{}", slice_2[0]); // print 1
println!("{}", slice_2[1]); // print 2
println!("{}", slice_2[2]); // print 3
```