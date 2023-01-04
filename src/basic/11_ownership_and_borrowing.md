# Ownership & Borrowing #

Sebelum kita membahas tentang *Ownership* dan *Borrowing*, kita akan membahas terlebih dahulu tentang dua jenis memori yaitu *Stack* dan *Heap*:
- *Stack*: adalah memori bersifat LIFO(*Last In First Out*) merupakan memori yang digunakan tempat menyimpan function frame(function stack frame) yang merupakan semua intruksi-intruksi pada suatu fungsi. Selain itu juga menyimpan tipe data primitif dan pointer address. Data yang dialokasi pada *stack* dapat diketahui size-nya pada saat compile time dan fixed in size selama program berjalan. Access cepat dan semua memori di-release ketika selesai digunakan(e.g function return/exit).
- *Heap*: adalah memori yang lebih kompleks dari stack. Memori ini digunakan untuk data yang tidak diketahui sizenya secara pasti pada saat compile time, dan bisa berubah-ubah size nya selama program berjalan. Memori ini butuh dynamic allocation pada saat runtime sehingga memiliki *overhead* dan butuh di-*deallocate* ketika sudah tidak digunakan. Di dalam Rust, proses de-*allocate* ini disebut dengan *Drop*.

## Ownership ##
Ownership merupakan hal unik pada Rust yang membuat rust mampu manage memory tanpa memiliki *Garbage Collector* serta tanpa manual memory management seperti *alloc/free*. Ownership memiliki 3 aturan:
- *Each value in Rust has an owner.*
- *There can only be one owner at a time.*
- *When the owner goes out of scope, the value will be dropped.*

Owner merupakan suatu variable yang memiliki sebuah value. Owner ini memiliki scope. Jika owner keluar dari scopenya, maka akan di-drop(released from memory).
Selain *drop*, ownership bisa berpindah tangan atau disebut dengan *move*, yaitu ketika owner masuk scope baru, sehingga owner tersebut tidak bisa digunakan di scope yang sama lagi.

Tipe data yang sering memiliki ownership adalah yang biasanya butuh alokasi *heap memory*, tidak fixed in size dan tidak diketahu size nya pada saat compile time.

Tipe data yang memiliki ownership terhadap valuenya adalah tipe data selain primitif.

Di dalam Rust terdapat 2 jenis trait yang berkaitan dengan *scope* variable:
- *Copy*: Semua tipe data primitif sudah meng-implementasikan *copy* secara implisit. Trait ini memungkinkan data di-pass atau keluar scope tanpa khawatir kehilangan ownership, Karena semua data tersebut memiliki struktur memori sederhana sehingga copy data terjadi dengan sangat cepat ketika keluar/berpindah scope.
- *Clone*: Tipe data selain primitif atau yang tidak meng-implementasi *copy* secara implisit, harus implement *clone* secara eksplisit. Jika data keluar/pindah scope, maka *ownership* akan di-drop/pindah. Tipe data yang *clonable* biasanya dialokasi ke dalam *heap memory*, sehingga harus melakukan *deep copy* terhadap struktur datanya.

### Scope ###
Scope adalah segment kode yang menandai batas ownership suatu data yang di-*owned*. Seperti dibahas di atas, terdapat dua mekanisme yang berkaitan dengan variable yang di-*owned* yaitu *drop* atau *move*. Dua jenis scope yang ada pada Rust yaitu *curly brackets*(drop) dan *functions/methods arguments*(move).

- Contoh drop:
```rust
fn main() {
    {
        let s = String::from("string value");
        // string s will be dropped here at the end of the scope
    } // this curly bracket mark the end of s owning the String value so be dropped.
}
```
*Drop* merupakan sebuah trait yang hampir semua tipe *owned*/*heap allocated* sudah implement secara implisit. Compiler *rustc* akan meng-*embed* drop function untuk data terkait di setiap end of scope.

- Contoh move:
```rust
fn main() {
    let s = String::from("hooh");
    function(s);
    println!("{}", &s);
    println!("{}", s);
}

fn function(s: String) {} 
```
Pada contoh di atas, variable `s` sudah berpindah scope dari `main` ke scope fungsi `function` sehingga `s` tidak bisa lagi digunakan setelah `function(s)`.
*Move* hanya terjadi pada tipe data yang bersifat *clonable* atau implement trait `Clone`. Tipe-tipe data ini biasanya dialokasi pada heap, contoh seperti di atas adalah `String`. Cara memanfaatkan data *clonable* setelah move terjadi adalah dengan memanggil method dari trait *Clone* itu sendiri yaitu `clone`. Sehingga code di atas menjadi:
```rust
fn main() {
    let s = String::from("hooh");
    function(s.clone());
    println!("{}", &s);
    println!("{}", s);
}

fn function(s: String) {} 
```
Sehingga `s` masih bisa digunakan setelah `function`, karena value dari `s` di-clone(eksplisit copy) ke dalam parameter `function`.
Clone melakukan *deep copy* untuk menyalin semua data yang ada ke memory baru, sehingga ini memiliki cost yang jauh lebih besar ketimbang *copy* biasa. Hal ini karena kebanyakan *copy* terjadi pada stack memory atau data dengan tipe sederhana(primitif) sehingga copy dan alokasi baru dapat dengan sangat cepat dan mudah. Untuk Clone kebanyakan data merupakan *heap allocated* sehingga butuh clone semua bentuk struktur data yang arbitrary dan mencari segment memory baru untuk alokasi baru. 
Untuk menghindari *overhead* ini, ada cara lain untuk mem-passing variable, yaitu dengan cara *Borrowing*.

## Borrowing ##
Borrow adalah ketika kita me-*reference* suatu data, disebut juga dengan *reference type*. Berbeda dengan *Owned* data, *borrowed* data tidak memindahkan ownership ketika memasukin scope baru. *Borrowed* data bisa berpindah2 scope bahkan ke dalam parameter fungsi lain sehingga data masih bisa digunakan di scope yang sama. 
Reference type berbeda dengan pointer type, dimana:
- Reference type: must not null, always valid state and value, can be operated like normal variable.
- Pointer type: can be null, unsafe, can cause program crash if dereferencing null pointer, butuh dereferencing saat ingin menggunakan valuenya.
Di dalam rust, kode normal sehari2 kebanyakan menggunakan reference type, jarang kita membutuhkan pointer karena konsep safety pada rust menghindari null pointer exception. Ada pengecualian untuk hal seperti butuh fleksibilitas management memory sehingga butuh pointer dan ini hanya bisa dilakukan dalam context *unsafe* pada Rust.

Semua tipe di dalam Rust bisa di-*borrowed* dengan cara menambah *ampersand*(&) di awal tipe seperti `&i32`. Ketika suatu tipe data di-*borrowed* tidak terjadi copy terhadap memory, akan tetapi *borrower* mereferensikan memory lokasi si data.

Borrow memiliki beberapa rules diantaranya:
- There can be **more** than one immutable borrow
- There can be **only** one mutable borrow
- Reference must always be valid

Contoh immutable borrowing:
```rust
fn borrow() {
    let anu = "anu";
    {
        println!("{}", anu); // move scope
    }

    println!("{}", anu); // still valid here
    accept_borrow(anu);
    println!("{}", anu); // still valid here
}

fn accept_borrow(s: &str) { // this function borrow s as reference of string(&str)
    println!("{}", s);
}
```

Contoh mutable borrowing:
Failed:
```rust
fn mutable_borrow() {
    let mut a = 123;

    let b = &mut a; // first mutable borrow
    let c = &mut a; // second mutable borrow

    {
        let d = &mut a; // third mutable borrow
    }

    println!("{}", b);
    println!("{}", c);
}
```
Pencegahan multiple mutable reference ini bertujuan untuk menghindari kemungkinan *data race*. Data race adalah ketika suatu data di lokasi memori yang sama diakses oleh lebih dari 1 reference/pointer baik dalam 1 thread yang sama atau dalam thread yang berbeda. Dalam thread berbeda sangat jelas kemungkinan data race karena kita masing-masing thread tersebut bisa berjalan secara *concurrent* yang menyebabkan *undefined behaviour* pada data yang diaccess dengan minimal salah 1 nya adalah proses *write*/*modify* data. Selain itu pengaksessan mutable dalam 1 thread juga bisa bermasalah karena masing2 reference bisa memiliki waktu proses yang berbeda terhadap data yang diakses, sehingga tidak ada sinkronisasi data yang pasti sehingga menyebabkan *undefined behaviour*.

*Immutable borrowing* digunakan ketika data yang di-passing tidak membutuhkan ownership dan berpindah ke berbagai thread.
*mutable borrowing* harus digunakan dengan mekanisme *mutex* untuk menjaga konsistensi data agar tidak terjadi *undefined behaviour*.

Borrowing dapat dengan mudah dilakukan ketika mem-*passing* data antar stack-frame fungsi2 yang *nested* ke dalam. Akan tetapi ketika ingin me-*return* data reference, hal ini tidak bisa dilakukan, apalagi pemilik asli data yang di-*borrowed* berada dalam stack-frame tersebut sehingga ketika data reference di-return, akan terjadi *use-after-free* atau semacam *dangling pointer* karena stack-frame sumber data reference tersebut sudah di-release dari stack memory. Bahasa-bahasa lain seperti Go menerapkan konsep *escape analysis* untuk menentukan lokasi data-data reference/pointer ketika keluar dari scope fungsi(stackframe). Dalam Go, ketika keluar dari stack-frame sedangkan data yang di-*pointed*/di-*referenced* ada di dalam stack frame tersebut, maka data tersebut dilarikan(*escape*) ke *heap memory* dan akan di-release oleh GC. Rust memiliki pendekatan lain untuk hal ini, yaitu dengan menggunakan *lifetime* yang akan kita bahas pada pembahasan selanjutnya.