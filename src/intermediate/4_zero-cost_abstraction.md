# Zero-cost Abstraction #

> *"What you don’t use, you don’t pay for. And further: What you do use, you couldn’t hand code any better."*
> 
> ~ Bjarne Stroustrup, [Foundations of C++](https://www.stroustrup.com/ETAPS-corrected-draft.pdf)

Zero-cost merupakan salah satu prinsip utama dalam Rust yang memiliki pengaruh dari C++. Definisi dari ZCA adalah meminimalisir *runtime overhead* sebanyak mungkin dengan melakukan banyak komputasi pada saat *compile-time*. ZCA menekankan pada abstraksi kode program yang memiliki performansi yang sama dengan kode program yang ditulis manual secara prosedural(*imperatively*). 

Ketika kita menulis program manually by hands, *step-by-step*, setiap iterasi, state, dan *wrapper* yang kita taruh akan kita pertimbangkan dengan sematang-matangnya tanpa mengenai performansi pada saat runtime. Hal ini tentunya sangat melelahkan karena terlalu manual, terlalu *menial* dan *repetitive*. Abstraksi dikenalkan pada pemograman supaya lebih memudahkan kita dalam merepresentasikan beberapa details mejadi satu konsep supaya lebih mudah diingat dan ditulis. Dengan abstraksi akan memudahkan dan mempercepat kita dalam menulis kode program karena semua hal-hal detail dan *irrelevant* sudah kita abstraksikan ke dalam abstraksi yang kita buat, sehingga tidak perlu dirisaukan lagi. Akan tetapi hal ini memiliki *trade-off* yaitu kita harus membayar kemudahan ini dengan *performance penaly*, sehingga kode program yang diabstraksi akan memiliki performansi lebih lambat ketimbang ditulis manual.

Berangkat dari sinilah muncul konsep ZCA ini, yaitu untuk mengoptimisasi dan mengefisiensi kode program tanpa mengeluarkan ekstra effort menulis dengan manual, dan tetap mendapat performansi yang sama dengan menulis manual *by hands*.

Rust memiliki prinsip ini dengan compiler-nya yang banyak memproses dan mengoptimisasi hal pada saat compile-time, sebisa mungkin banyak hal bisa ditangkap pada saat compile-time, sehingga semua kemungkinan bugs bisa ditangkap pada saat compile-time, dan menyisakan sangat sedikit bahkan *free* untuk runtime. `rustc` dengan `llvm` bekerja sama dalam meng-*deconstruct* dan mengoptimisasi semua abstraksi kode sehingga hasil build akhir sama dengan kode yang ditulis manual *by hands* dan memiliki performansi yang mirip.

Berdasarkan [paper ini](https://www.stroustrup.com/ETAPS-corrected-draft.pdf) ada beberapa hal yang berkaitan dengan ZCA diantaranya resource management, compile-time computation, error handling, concurrency dan performance. Selain itu juga berkaitan dengan bagaimana kaitannya dengan type system dari suatu bahasa. Kita akan membahas ini kaitannya dengan Rust.

Berikut hal-hal yang berkaitan dengan ZCA di Rust:

## Static Type Analysis ##
Rust merupakan bahasa yang memiliki static dan strongly typed type system. Static Type pada Rust menerapkan *affine* dan *linear* types dibarengi dengan RAII serta [Hindley–Milner type system](https://en.wikipedia.org/wiki/Hindley%E2%80%93Milner_type_system) untuk full type inference. Hal ini mengurangi kewajiban untuk mendeklarasikan tipe di beberapa tempat karena akan langsung di-*inferred* oleh compiler.

Strong type menghasilkan type safety dengan berbagai cara, di antaranya: no null pointer, always valid reference, immutable by default, no RTTI, no reflection, bound checks, no type casting, no type assertion, dan lainnya. Setiap type memiliki state yang jelas serta memiliki scope/lifetime yang jelas juga. Dibarengi dengan RAII Rust memiliki jaminan terhadap invariants pada program dengan memanfaatkan type systemnya untuk menjaga dari setiap invariant violations.

Hal-hal di atas memungkinkan compiler mengetahui dan mengantisipasi beberapa *pitfalls* yang mungkin terjadi pada saat runtime.

## No Garbage Collector ##
Memiliki manajemen memori tanpa Garbage Collector. Garbage collector merupakan salah satu runtime-cost yang memiliki cost berupa memory management dan segala overheadnya, seperti CPU usage untuk GC bekerja, memory management dan safety memory measures, *compacting*, *stop-the-world*, dan lainnya yang berkaitan dengan memory management. Bahasa-bahasa yang memiliki GC tentunya memiliki alasan agar memory management tidak manual seperti C dan C++. Hal ini tentunya membawa cost berupa runtime overhead. 

Rust menerapkan prinsip ZCA dengan cara menghilangkan GC, dan tanpa harus mengatur memory secara manual seperti di C/C++, yaitu dengan menerapkan konsep **Ownership dan Borrow**, dimana Rust men-*tracking* setiap object yang di-*create* beserta lifetime-nya. Hal ini menghasilkan dua jenis data yang ada, yaitu: 1. *Exclusive data*, dan 2. *Shared data*. 

*Exclusive* data merupakan data yang di-*owned*, biasanya data yang tidak diketahui *size* nya pada saat compile time(heap allocated), sistem ownership diterapkan pada exclusive data dengan cara melakukan binding hanya pada 1 owner untuk setiap data di satu lifetime/scope. Ketika lifetime habis atau scope berpindah, terjadi dua kemungkinan: data berpindah ownership(*move*), atau data di-*drop*. Eksklusivitas dari data ini memberikan handle yang aman terhadap data dari segala memory issue dan juga terhadap concurrency issue. Selain itu juga memberikan kemampuan GC tanpa GC.

*Shared* data merupakan data yang di-*borrowed*, biasanya data yang berupa reference type(&). *Borrow* bisa bersifat *immutable* dan *mutable*. *Immutable borrow* dilakukan untuk memberikan data yang bersifat read-only, tanpa melakukan *copy* terhadap original data. *Mutable borrow* dilakukan ketika ingin memberi *side-effects* terhadap suatu data, tentunya juga tanpa *copy* terhadap original data. *Borrow* sangat penting ketika data yang diolah cukup besar dalam size, dan tidak efisien ketika di-*copy*, sehingga cukup dengan mengirim *reference* terhadap data tersebut. Rust menerapkan *borrow checker* untuk memastikan semua lifetime reference type valid dan data yang direferensikan valid, pada saat compile-time tentunya.

## Data Race Check ##
Thread safety merupakan fitur yang didapatkan setelah menerapkan type dan memory safety. 

Bahasa lain seperti Golang, memiliki feature race detector untuk mendeteksi data race. Akan tetapi feature ini tidak ZCA karena kita harus membayar cost yang sangat besar jika ingin menggunakannya(build dengan flag `-race`). Menurut situs official [ini](https://go.dev/doc/articles/race_detector#Runtime_Overheads), data race detector pada Golang bisa memberikan runtime overhead sebesar 5-10x memory usage dan 2-20x execution time. 

Rust dengan konsep memory safety dan ZCA dapat mendeteksi data race saat compile-time tanpa overhead saat runtime. *Check* ini dilakukan pada saat compile time dengan memeriksa *ownership* suatu data, dengan memanfaatkan *borrow checking* setiap references/pointers yang di-share secara mutable antar threads akan gagal di-compile.

## Zero-cost Async with Future ##

Rust pada awalnya menerapkan [greenthread](https://cfsamson.github.io/books-futures-explained/0_background_information.html#green-threadsstackful-coroutines) untuk runtime asynchronousnya, tetapi di-remove sebelum versi 1.0. Beberapa alasan di-remove di antaranya greenthread memiliki cost runtime, stackful, stateful, need to store CPU state inside memory in each context switch, kalau ingin FFI maka butuh switch context dan thread lebih besar lagi, karena beda call semantics antar bahasa. 

Akhirnya metode asynchronous yang diterapkan adalah Future-based, yaitu pendekatan functional terhadap asynchronous code. Future bekerja layaknya state machine yang di execute oleh semacam *executor*(provided by async library e.g. Tokio). Future sebenarnya adalah monad yang mengkombinasikan closure2 yang di-execute lazily. Dari sudut pandang user kode terlihat asynchronous dengan syntax sugar async/await, akan tetapi ketika di-compile, future akan dikonversikan ke semacam state-machine yang saling sinkron dengan state-state yang ada(pakai enum, dengan state2 seperti wake, pending, finished, etc), jadi tidak ada cost context switch pada level runtime, akan tetapi pada level code oleh type system.

Berikut contoh sederhana perbandingan async pada go dan rust:
Program berikut melakukan looping terhadap fungsi yang dijalankan secara asynchronous.

Golang:
```go
package main

import (
	"fmt"
	"time"
)

func main() {
	start := time.Now()
	for i := 0; i < 100000; i++ {
		go nothing()
	}
	end := time.Since(start)
	fmt.Printf("elapsed: %v", end)
}

func nothing() {}
```
**Go Result:**: `elapsed: 29.846477ms`

<br/>

Rust:
```rust
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    for i in (0..100000) {
        nothing().await;
    }
    let end = start.elapsed();
    println!("elapsed: {:#?}", end);
}

async fn nothing() {}
```
**Rust Result**: `elapsed: 56.724µs`

Pada benchmark sederhana di atas Rust lebih cepat sekitar 500x lebih cepat. 

Microbenchmark ini mungkin tidak berarti apa-apa untuk real life application. Tapi disini bisa dilihat gimana cost untuk spawn *goroutine*(greenthread), dibanding pendekatan *future* yang dilakukan Rust secara zero-cost, tanpa melakukan apapun pada fungsi yang dipanggil.

Go memiliki cost *spawning* pada saat runtime dengan ukuran stack awal 2KB, yang bisa grow sesuai kebutuhan. Bayangin setiap iterasi melakukan ada stack switching ke stackframe nya goroutine. Pada rust semua futures chain di jadikan fungsi-fungsi yang sinkron layaknya state machine tanpa switch stack karena stackless coroutine.

## Generics & Traits ##
Generic pada Rust bersifat monomorphism, dimana terjadi monomorphization terhadap fungsi yang generic menjadi fungsi-fungsi yang lebih spesific sesuai dengan boundary type parameters. Semua ini merupakan fungsi stand-alone dan bisa dioptimisasi oleh compiler pada saat compile-time. Bahasa lain seperti Java menerapkan *type-erasure* dimana type dihapus dan diganti dengan object class yang mana semua class inherit dan generics pada java sebenarnya meng-automatisasi casting ke object class ini.

Monomorphization ini sebenarnya hal yang cukup *straightforward*, contoh:
Tanpa generics:
```rust
fn do_i32() {
    ...
}

fn do_i64() {
    ...
}

fn do_f32() {
    ...
}

fn do_f64() {
    ...
}
```

Dengan generics:
```rust
fn do<T: Sized + Copy>(param: T) {
    ...
}
```
Generics dibarengi dengan traits, dapat memberikan polimorfisme pada fungsi/method tanpa runtime cost seperti pada interface di bahasa lain(Go, Java, etc).
Sized dan Copy merupakan marker traits untuk sifat-sifat suatu tipe data, Sized merupakan traits untuk tipe yang diketahui sized nya pada saat compile time dan fixed, serta Copy merupakan traits untuk tipe data yang dapat dicopy bit-by-bit dengan sangat murah, misal primitive types. Semua primitive types sudah implements Copy secara default. Fungsi di atas akan di-konversi ke beberapa bentuk di atas dan di optimisasi oleh compiler(e.g inlining).

## Static Dispatch ##
Rust memiliki handle untuk polymorphism yang bisa melakukan method dispatches statically. Dispatch jenis ini memproses tipe data dengan trait bersangkutan pada saat compile-time dan memastikan semua matched dan tanpa menyimpan states pada saat runtime(e.g. pointers and vtables).

Interface pada Golang merupakan semacam *fat pointer* dimana menyimpan reference untuk DST(Dynamically Sized Type) seperti concrete data dan methods. Ketika kita melakukan passing data ke dalam atau ke luar interface, data tersebut akan escape ke heap karena interface yang digunakan akan melakukan lookup ke vtable pada saat runtime ketika interface dipanggil/assert. Proses lookup ini membutuhkan runtime overhead dan tidak cacheable oleh CPU karena butuh call ke ranah memori untuk data dan function signatures.

Rust melakukan static dispatch dengan memanfaatkan trait. Kita bisa menggunakan syntax `impl Trait` atau dengan notasi generic.
Contoh:
```rust
pub trait Animal {
    fn walk(&self);
    fn sleep(&self);
    fn eat(&self);
}

fn static_dispatch<A: Animal>(a: A) {
    a.walk();
    a.sleep();
    a.eat();
}

fn static_dispatch_impl(a: impl Animal) {
    a.walk();
    a.sleep();
    a.eat();
}

struct Cat;

impl Animal for Cat {
    fn walk(&self) {
        println!("cat is walking...!")
    }
    fn sleep(&self) {
        println!("cat is sleeping...!")
    }
    fn eat(&self) {
        println!("cat is eating...!")
    }
}

fn main() {
    let c = Cat;
    static_dispatch(c);
    static_dispatch_impl(c);
}
```
Ketika `c` di-pass ke dalam `static_dispatch` atau `static_dispatch_impl`, akan terjadi monomorphization terhadap `c` sesuai dengan Trait yang diimplementasikan.

## Iterator ##
Pattern iterator bisa diterapkan dengan sedemikian abstraksi dan ketika di-compile stripped-down layaknya *optimized hand-written* code dengan manual looping, seperti di C. 
Contoh:
```rust
pub fn main() -> i32 {
    sum() + sum() + sum() + sum() + sum() + sum() + sum() + sum()+ sum() + sum()+ sum() + sum()
}

pub fn sum() -> i32 {
    vec![1, 2, 3].into_iter().reduce(|x, y| x + y).unwrap()
}
```
Kode di atas jika dilihat di x86-64 menjadi:
```asm
example::main:
        mov     eax, 72
        ret

example::sum:
        mov     eax, 6
        ret
```

Kita bisa lihat bahwa pada asm di atas hasil dari komputasi fungsi `sum()` sudah diketahui hasil nya dan penjumlahan dari beberapa fungsinya juga telah diketahui hasil nya. Jika kita bandingkan dengan bahasa lain terdapat proses komputasi pada saat looping, dan juga setiap ekspresi pemanggilan fungsi juga terjadi komputasi satu-per-satu. Abstraksi iterator yang kita buat di atas di-kompilasi sedemikian rupa sehingga memiliki performa yang sama dengan *highly optimized hand-written code*.
Ini cuma salah satu dari banyak contoh dalam iterator pattern yang bisa dieksplor.

## Zero-sized Types ##
Kita bisa mendeklarasi tipe data cuma sebagai marker dan tidak memakan resource pada saat compile-time. Ini digunakan sebagai bagian dari analysis program secara statis.
Konsep *ownership* dan *borrowing* masih berlaku pada saat program di-kompilasi sehingga secara semantic masih berlaku sama dan semua *markers* tersebut di-*stripped* pada saat compile-time.

## Macros ##
Macro adalah salah satu feature Rust untuk meng-ekstensi syntax bahasa. Macro juga digunakan untuk mengurangi beberapa hal yang bersifat *repetitive* ketika koding, dengan meng-automatisasi syntax generations dan juga melakukan type checks, *at compile-time*.

Macro juga feature yang sangat significant untuk menulis DSL.

## Auto-Vectorization ##
Rust memiliki optimisasi kompilasi dimana compiler menerapkan SIMD(*Single Instruction, Multiple Data*). Diantara optimisasi yang diterapkan secara default adalah SIMD SSE dan SSE2 pada CPU Intel x86. Masih banyak optimisasi lain yang terjadi, tapi SIMD lah salah satu optimisasi yang cukup significant yang dapat meningkatkan performa program tanpa effort dari programmer, tanpa harus optimisasi manual *hand-coded*, tanpa harus menghilangkan abstraksi pada code. 

Salah satu contoh SIMD adalah contoh pada iterator di atas, dimana di dalam looping dan iterator tidak terjadi *actual looping* jika dilihat pada baris kode, akan tetapi semua dijadikan 1 vektor data yang dieksekusi oleh 1 instruksi CPU. Kita bisa bandingkan ini dengan hasil asm dari bahasa lain.

<br/>

Mungkin masih banyak jenis ZCA yang ada di Rust yang belum di-cover di sini. 
Tanpa ZCA, developers biasanya berdebat antara membuat code yang *maintainable* atau membuat code yang *performant*. Dua hal ini sering bertentangan satu sama lain karena *maintainability* menerapkan banyak abstraksi pada kodenya demi readibility, clarity, dan kemudahan untuk developer dalam implementasi fungsionalitas program tanpa *hand-coding*. Sedangkan sebagian developers juga memiliki *concern* terhadap *performance* dari code mereka, sehingga tidak jarang abstraksi dihilangkan dan menghasilkan kode yang kurang maintainable dengan readibility yang hanya bisa dipahami oleh penulis awal program tersebut.

Dengan ZCA, hal ini dapat teratasi karena developer masih bisa menerapkan abstraksi pada kode mereka untuk menjaga maintainability dengan sedemikian abstraksi, ketika program di-kompilasi, compiler akan mengoptimisasi kode dengan salah satunya menerapkan ZCA untuk menghilangkan abstraksi yang tidak relevan pada saat runtime sehingga performansi program tetap terjaga.