# Pengenalan #

Rust adalah bahasa pemograman yang di-*release* ke publik tahun 2010 berawal dari projek riset Mozilla dan didesain oleh Graydon Hoare. Bahasa ini memiliki *static* dan *strong type*. Rust merupakan bahasa yang meng-*enforce* *type safety* dan *memory safety*. Kedua hal ini berhubungan erat satu sama lain yang akan meng-*handle* hal-hal yang biasa terjadi di bahasa lain, di antaranya:
- *Dangling pointer*, adalah ketika suatu *pointer* tidak menunjuk ke value yang valid di memory. Biasanya ini terjadi ketika *free* memory tidak dilakukan di tempat atau waktu yang tepat.
- *Double-free*, melakukan *free* memory lebih dari 2 kali untuk lokasi memory yang sama. Ini biasa terjadi pada bahasa yang mana manajemen memori nya manual, seperti C/C++.
- *null pointer*, terjadi ketika mengakses pointer yang kosong.
- *data-race*, terjadi ketika data diakses oleh lebih dari satu *thread* secara konkuren. Hal ini bisa menjadikan data tidak konsisten khususnya ketika ada proses write dan read terhadap data tersebut.
- *memory-leaks*, terjadi ketika banyak resources yang sudah tidak diperlukan akan tetapi masih mendiami memory sehingga memakan resource yang diperlukan oleh proses lain.
- dan lainnya.

Rust merupakan bahasa *compiled* ke *binary* sehingga berjalan secara *native* di OS target. Compiler Rust bernama `rustc` yang secara simple terdiri dari *frontend* yang menghasilkan IR(*Intermediate Representation*) dan *backend* menggunakan LLVM yang menghasilkan hasil akhir berupa binary.

Rust memiliki konsep baru di dunia pemograman yaitu: *Ownership* dan *Borrowing*. 
*Ownership* adalah dimana setiap value di dalam rust memiliki 1 owner dalam 1 waktu dan tempat. *Ownership* berpindah ketika value tersebut keluar dari tempatnya/*scope*-nya. Scope disini bisa berbagai macam bentuk, mulai dari codeblock dengan `{}`, fungsi, lambda, dan assignments. *Borrowing* adalah ketika kita ingin meng-*passing* value tanpa memindahkan *ownership*, yaitu dengan memberikan *reference* kepada value yang kita *passing*. Kedua feature inilah yang banyak meng-*handle* *memory safety* yang sering terjadi di atas.

Hal unik lainnya adalah cara Rust manajemen memori. Rust tidak memiliki manual memori manajemen seperti C dan C++ serta tidak juga memiliki *Garbage Collector*. Rust mengatasi ini dengan memanfaatkan 2 konsep di atas untuk meng-*inject* pelepasan *resource* memori di dalam code pada saat compile time. Ketika resource yang di-*owned* keluar dari scope terjadi 2 kemungkinan: 1. Berpindah kepemilikan, atau 2. Di-*release* dari memori ketika tidak ada yang me-*reference* dirinya. Dengan begini penggunaan *resource* memori sangat efisien dan ringan tanpa *runtime overhead* seperti GC.

Kelebihan Rust:
- Type safety, Memory safety, dan Thread safety
- Performansi menyerupai C dan C++
- Memiliki automatic memory management tanpa GC
- Friendly compiler error message
- Cargo!
- Safe Concurrency

Kekurangan Rust:
- Learning curve yang tinggi.
- Kompilasi yang cukup lama ketika ukuran projek semakin besar.