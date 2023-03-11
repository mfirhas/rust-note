# Functional Programming pada Rust #

Rust memiliki berbagai macam paradigma pemograman yang dapat dicampurkan ke dalam kode, yaitu: Procedural/Imperative dan Functional/Declarative. Procedural merupakan pendekatan bahasa pemograman yang paling awal muncul memiliki pengaruh langsung dari konsep Turing machine itu sendiri. Selain itu juga bahasa assembly juga secara keseluruhan bersifat imperative dan procedural. Pendekatan ini menekankan penulisan kode program secara berurutan step-by-step dari atas ke bawah dengan perubahan state di antaranya.

Pendekatan kedua yang muncul yaitu Functional yang memiliki pengaruh dari Lambda Calculus oleh Alonzo Church. Pendekatan kedua ini menggunakan fungsi sebagai dasar komputasi yang bisa dikombinasikan dan digabungkan menghasilkan abstraksi yang lebih tinggi untuk melakukan komputasi dengan menekankan *immutability* dari data.

Rust dengan type system nya bisa mengaplikasikan pendekatan functional dengan memanfaatkan 3 komponen utama:
- [Generic types](../basic/15_generic.md)
- [Traits](../basic/14_trait.md)
- [Closures](../basic/5_variables_function_closure.md#closures)

Rust mendapatkan polymorphic type parameters secara static dan strong oleh generics dan traits, serta dapat mengembangkan komputasi menggunakan closures.
Kita telah membahas ketiga di atas pada pembahasan sebelumnya. 

Rust tidak murni functional, akan tetapi banyak dari praktiknya menggunakan paradigma functional.

Kita akan membahas kaitan konsep-konsep pada functional programming di dalam Rust.

## Types & Immutability ##
Rust memiliki semua data yang bersifat immutable by default. Ini mencegah *side-effects* yang mungkin terjadi terhadap data yang di-*pass* ke berbagai fungsi, atau bahkan thread lainnya. Hampir semua tipe data di dalam Rust bersifat monoid dan bisa membentuk tipe baru(category). 
Berbeda dengan beberapa bahasa lain yang beberapa variable memiliki default values, dan beberapa memiliki undefined value, atau bahkan null. Di dalam Rust hampir semua merupakan object matematika berupa set atau kategori. 

Semua tipe data merupakan set itu sendiri, karena tidak ada empty values padanya, dan default values yang harus ditentukan manual yang mana value nya juga bagian dari set itu. Dengan memanfaatkan generics dan trait, kita bisa membentuk type constructor/container untuk membuat tipe baru, atau disebut juga sebagai category. Semua ini bersifat immutable by default.

## Pure Function ##
Pure Function tidak memiliki side-effects yang reflected ke luar fungsi. Jikalau Pure Function melakukan perubahan, perubahan itu hanya local state dari fungsi tersebut tanpa reflected ke luar. Hal ini berbeda dengan beberapa idiom yang biasa terjadi di C dan Go dimana kita mem-*passing* pointer ke dalam fungsi yang akan merubah state dari pointer tersebut.
Karena semua immutable by default, maka variable yang di-pass ke dalam juga immutable by default. Kondisi variable yang di-pass ke dalam fungsi mengikuti deklarasi variable di parameternya.
```rust
fn main() {
    let mut a = 20;
    accept_pure(&a); // pass immutable reference
    accept_pure(&mut a); // pass mutable reference
    accept_not_pure(&mut a); // pass mutable reference
    // accept_not_pure(&a); // can't pass immutable arguments into mutable parameter
}

// pure function
fn accept_pure(a: &i32) { // whatever the condition of arguments passed, those will follow function's parameters declaration.
    println!("pure function: {a}");
}

// non-pure function
fn accept_not_pure(a: &mut i32) {
    *a += 1;
    println!("non-pure function: {a}");
}
```
Pada contoh di atas, terdapat rules yang bisa ditulis:
- Immutable parameter bisa menerima immutable dan mutable arguments, karena fungsi tidak memiliki side-effects karena immutability dari parameternya.
- Mutable parameter hanya bisa menerima mutable argument, karena fungsi ini pasti memiliki side-effects karena mutability dari parameternya.

Dua rule di atas digunakan untuk memberikan abstraksi yang jelas dari API-API yang kita buat.

## First-order function ##
Rust menerapkan first-order function melalui closure, yang mana seperti yang telah dibahas sebelumnya, closure-closure pada rust menerapkan salah satu dari 3 trait closures yang telah dibahas di [sini](basic/../../basic/5_variables_function_closure.md#closures), yaitu `FnOnce`, `FnMut`, dan `Fn`.
First-order function yang bersifat pure adalah `FnOnce` dan `Fn` karena tidak memiliki side-effects terhadap data dari luar.
First-order functions di dalam rust sering digunakan sebagai fungsi transformasi yang di-inject ke dalam fungsi kombinator(higher-order function).
```rust
fn main() {
    let double = |x| x * x;
    do_double(double);
}

fn do_double<F: FnOnce(i32) -> i32>(f: F) {
    println!("{}", f(5)); // 25 
}
```

## Higher-order function ##
Merupakan fungsi kombinator yang mengkombinasikan satu atau beberapa first-order functions menjadi value atau fungsi lainnya. Bisa juga disebut sebagai kombinator.
Di dalam Rust, kombinator banyak digunakan untuk proses komputasi terhadap data yang ada di dalam container/type constructor, seperti Vec\<T>, Option\<T>, Result<T,E>, dan lainnya. Semua tipe container itu memiliki method-method yang merupakan functors, yang mana functor-functor ini bisa menerima fungsi lainnya, yang akan diproses secara *lazy*, dan akan menghasilkan category lainnya yang juga memiliki functor-functor. Contoh Higher-order function yang sering kita jumpai adalah iterator.
Iterator merupakan trait yang berisi method-method untuk melakukan operasi terhadap data berbentuk collection.
```rust
let v = vec![1, 2, 3, 4];
let v_doubles = v.iter().map(|x| x * x).collect::<Vec<_>>();
println!("{:?}", v_doubles);
```
Pada contoh di atas, variable `v` merupakan collection berbentuk Vector, yang berisi integer 32. Method `iter()` menghasilkan category `Map` yang memiliki functor `map` yang mengkombinasikan closure/arrow `|x| x * x`. Selain itu juga memiliki method `collect()` yang mengembalikan data ke category yang diinginkan yaitu `Vec<_>`.

Contoh lain adalah:
```rust
let opt: Option<T> = Some(5);
let ret = opt.map(|x| x*x).unwrap_or(Default::default());
println!("{}", ret);
```
Option merupakan category yang memiliki value `None` atau `Some(T)`. Option memiliki functor map yang menerima fungsi untuk data yang ada di dalam `Some(T)`, jika tidak ada maka akan mengembalikan `None`.

## Referential Transparency ##
Immutability by default membuat penerapan referential transparency menjadi lebih mudah. Selain immutability, sistem ownership pada Rust juga membuat referential transparency menjadi trivial. Functional traits seperti `FnOnce` dan `Fn` yang bersifat immutable, tidak memberikan side-effects keluar dari program di luar dari ekspresi yang kita gunakan. Ekspresi akan menghasilkan output yang selalu sama karena tidak adanya side-effects yang menyebabkan perubahan hasil evaluasi ekspresi.

## Currying ##
***Currying*** merupakan salah satu fitur yang digunakan dalam functional programming. Fitur ini adalah bagaimana membangun chain of functions dengan cara menyediakan fungsi yang mengembalikan fungsi lainnya, dan seterusnya. 

Di dalam Rust, currying dapat di-achieve dengan menggunakan functional traits dan *boxing*(kita bahas di pembahasan selanjutnya tentang smart pointers).
Secara technical, currying mengkomposisi beberapa fungsi yang digunakan sebagai arguments dan returns. Fungsi ini merupakan pointer ketika tidak bisa di-*inline* oleh compiler. Inline merupakan proses menanamkan semua intruksi fungsi ke dalam fungsi yang memanggil fungsi tersebut. Pada saat currying tentunya inline cukup sulit dilakukan karena pada saat kompilasi, hanya fungsi terluar yang mungkin bisa di-*inline*, akan tetapi fungsi berikutnya tidak bisa di-*inline*. Untuk mengatasi ini, kebanyakan bahasa pemograman menerapkan fungsi tersebut sebagai pointer.

Jika kita ingin menerapkan inlined currying, menggunakan notasi closures langsung di assign ke sebuah variable, maka semua bisa di-*inlined* oleh compiler pada saat compile time.
Cotnoh:
```rust
let func = |x| {
    move |y| {
        move |z| {
            move |a| {
                println!("{a}");
                (x * y * z) + a
            }
        }
    }
};
println!("{:?}", func(3)(9)(2)(2));

let fullname = |firstname| {
    move |middlename| move |lastname| format!("{} {} {}", firstname, middlename, lastname)
};

println!("{}", fullname("Muhammad")("Fathir")("Irhas"));
```
Notasi closures di atas menggunakan `move` semantics, hal ini untuk memindahkan ownership value dari fungsi sebelumnya ke fungsi berikutnya, hal ini karena closures by default meng-*capture* values sekitar by borrow/reference, sehingga akan ada mismatch antara lifetime closures yang disusun. Oleh sebab itu, kita butuh memindahkan ownership tersebut supaya closure berikutnya bisa menggunakan values dari closures sebelumnya sekalipun closure tersebut nge-*outlive* lifetime closures sebelumnya.

Jika kita ingin menggunakan currying pada named function, jika hanya memiliki 1 return fungsi, maka tidak ada masalah. Akan tetapi masalah muncul ketika kita ingin me-*nested* kan lebih dari 1 return fungsi, dalam artian 1 return fungsi tersebut juga akan mengembalikan fungsi lainnya, dan seterusnya. Masalah ini terjadi ketika kita menggunakan notasi `impl Trait` terhadap argument dan/atau return values. 

Rust memiliki sebuah rule untuk `impl Trait` ini yaitu ***impl Trait* hanya bisa digunakan sebagai argument dan return langsung dari fungsi bersangkutan.**
Dalam artian, jika kita juga menggunakan `impl Trait` untuk input dan output dari `impl Trait` lainnya, maka ini melanggar aturan dan failed to compile.
Karena closures di dalam rust itu adalah jelmaan dari traits `FnOnce`, `FnMut`, dan `Fn`, maka aturan `impl Trait` juga berlaku untuk closures menggunakan notasi `impl Trait`. 

Kembali ke permasalahan di atas, yaitu ketika kita ingin mengembalikan nested currying lebih dari 1, jika menggunakan `impl Trait` saja tidak bisa, sehingga kita harus mem-*boxing* functional traits lainnya ke dalam smart pointer box dan menggunakan keyword `dyn`(kita bahas di pembahasan selanjutnya).
Contoh:
```rust
fn main() {
    println!("{}", currying1(1)(2));
    println!("{}", currying3(1)(2)(3));
}

fn currying1(first: i32) -> impl Fn(i32) -> i32 {
    move |second| first * second
}

// failed because it against the rule of `impl Trait` placement.
// fn currying2(first: i32) -> impl Fn(i32) -> impl Fn(i32) -> i32 {
//     move |second| move |third| first * second * third
// }

fn currying3(first: i32) -> impl Fn(i32) -> Box<dyn Fn(i32) -> i32> {
    move |second| Box::new(move |third| first * second * third)
}
```
Kenapa kita tidak bisa menggunakan notasi `impl` pada return fungsi ke tiga?, ini ada hubungannya dengan `static dispatch` pada Rust dimana `impl Trait` merupakan tipe untuk melakukan static dispatch(kita bahas pada pembahasan selanjutnya). Karena fungsi ke tiga tidak bisa di-*infer* secara static, maka butuh di-dispatch secara dynamic menggunakan kombinasi Box dan dyn(kita bahas selanjutnya).

## Functional Constructs pada Rust ##
Kita akan menerapkan beberapa konsep functional di atas ke dalam Rust menggunakan 3 komponen utama: Generics, Traits, dan Closures.
- Generics: Merupakan fitur umum yang ada pada functional programming. Fitur ini diperkenalkan oleh bahasa ML. 
  Jika kita memperhatikan notasi matematika, khususnya dalam bidang set dan kategori, kita melihat bahwa set(types) selalu dinotasikan menggunakan 1 huruf kapital,
  seperti A, B, C, T, U, X dan lainnya[[1]](https://en.wikipedia.org/wiki/Set_(mathematics)#How_sets_are_defined_and_set_notation). 
  Generics merupakan representasi dari notasi *set* di dalam matematika untuk merepresentasikan berbagai set(type) yang bisa digunakan.
- Traits: Digunakan untuk memberikan common things shared between sets, atau dalam hal ini type data yang telah di-generic-kan di atas.
  Ketika kita menggunakan generic tanpa trait, maka literally semua sets bisa saja dimasukkan ke dalam generic tersebut, untuk membatasi ini, maka bisa digunakan traits.
- Closures: Digunakan untuk merepresentasikan arrows pada matematika, yaitu memberikan kemampuan untuk meng-*inline* deklarasi fungsi pada kode. 
  Ini nantinya berguna untuk membangun first-order function, dan dikombinasikan ke dalam higher-order functions.

Contoh:
```rust
fn main() {
    let ret = functor(123, |x| x * x);
    println!("{ret}");
}

fn functor<T, F>(t: T, f: F) -> T
where
    T: Sized + Debug,
    F: Fn(T) -> T,
{
    println!("{t:?}");
    f(t)
}
```

Pada contoh Di atas, kita secara sederhana menerapkan 3 komponen di atas ke dalam functional approach.
Kita menggunakan generic sebagai type placeholder untuk fungsi `functor`, yang mana fungsi ini akan menerima 2 parameter `t: T` dan `f: F` yang mana masing-masing harus memenuhi trait `Sized dan Debug` dan functional trait `Fn(T) -> T`. Kita menggunakan functional trait Fn untuk menerima pure function sebagai first-order function(arrows) yang akan diproses lazily oleh `functor`.


## Monad ##
Kita akan menelusuri kembali semua konsep berkaitan dengan monad yaitu:
- Set: Semua tipe data pada Rust merupakan *set* di dalam abstract algebra. Contoh: primitive types: `i32`, `f32`, `bool`, `char`, etc
- Monoid: Semua tipe data di Rust merupakan Monoid, karena semua bisa bersifat *associative* terhadap suatu binary operation, dan memiliki identity element.
  Contoh: (i32, +, 0), i32 adalah monoid untuk operasi penjumlahan dengan identity element 0.
- Category: Merupakan tipe container atau *type constructor* yang terdiri dari Set/Monoid itu sendiri. Contoh: `Vec<T>`, `HashMap<K,V>`, etc.
- Arrows: Merupakan first-order functions jelmaan dari functional traits: `FnOnce`, `FnMut`, dan `Fn`.
- Functor: Merupakan mapping dari suatu category ke dalam bentuk lain, khususnya ke dalam category yang sama jika endofunctor. 
  Functor sendiri bisa associated functions dari suatu category, bisa juga Category tersendiri dengan beberapa methods yang bertugas melakukan *mapping*/transformasi category.

> Monad di dalam Rust merupakan tipe data berbentuk **Category** yang berisi **Monoid**, yang memiliki **endofunctor** yang melakukan mapping terhadap monoid tersebut menggunakan **arrows** yang dikombinasikan ke dalam endofunctor.

Beberapa contoh monad di dalam Rust yang sering kita gunakan:
- `Option<T>`: Option, berisi 1 monoid T, memiliki beberapa endofunctor di antaranya `.and_then(F)` dan `map(F)`. Masing-masing bisa memetakan `Option<T>` ke category yang sama dengan tipe yang sama yaitu `Option<U>`. Bisa juga memetakan ke tipe yang berbeda tetapi masih di dalam category yang sama, misal `Vec<T>`.
Contoh:
```rust
let opsi: Option<&str> = Some("test");

// map the monoid inside option if it's exist, by injecting it into arrows F: |x| -> x
let ret = opsi.map(|x| x); //.or(None);
println!("{:?}", ret); // prints "Some("test")"

// do something with Option if it's exist, and return Option<U>
let ret = opsi.and_then(|x| Some(x)); //.or(None);
println!("{:?}", ret); // prints "Some("test")"

// Option<T> dan Vec<T> bisa dikategorikan ke dalam Category yang sama, karena Option adalah vector dengan 1 elemen, atau empty vector.
let ret = opsi.into_iter().map(|x| x).collect::<Vec<_>>();
println!("{:?}", ret); // prints ["test"], will print [] if opsi is None
```

- `Vec<T>`: Vec, berisi 1 monoid T, memiliki beberapa endofunctor di antaranya `iter()`/`into_iter()` untuk menghasilkan functor mapping object dari vector.
Contoh:
```rust
let vec = vec![1, 2, 3];
// iter() get functor `Map` that containes the endofunctors for mapping the items inside vector, with `map` along with arrows, and collect it back into same category.
let ret = vec.iter().map(|x| x * x).collect::<Vec<_>>();
println!("{:?}", ret);

// Collect into `LinkedList<_>` because Vec and LinkedList have same category(shape: list of things)
let ret = vec.iter().map(|x| x * x).collect::<LinkedList<_>>();
println!("{:?}", ret);
```

- `HashMap<K,V>`: Berisi monoid berupa compound data of K and V, memiliki endofunctors di antaranya `iter()` and `into_iter()`.
```rust
let vec = [(1, "1"), (2, "2"), (3, "3")];
// vector of array of tuple-2 have same shape(category) with HashMap itself.
// need to use `into_iter()` to get functor owning the data from vector, since vector contain reference data(&str), and HashMap is owned type and need ownership of the data it collect from.
let ret = vec.into_iter().map(|x| x).collect::<HashMap<i32, &str>>();
println!("{:#?}", ret);

// failed because HashMap need to take ownership of iterator elements.
// hence HashMap cannot be built from &(K,V) because it's not implemented for FromIterator(&(K,V))
// let ret = vec.iter().map(|x| x).collect::<HashMap<i32, &str>>();
// println!("{:#?}", ret);

let tuple = [(1, 1), (2, 2), (3, 3)];
// we can use `iter()` insetead of `into_iter` because tupe values are all copyable, so no reference borrowed and move semantic occurs since i32 is copyable.
let ret = tuple.iter().map(|x| *x).collect::<HashMap<i32, i32>>();
println!("{:#?}", ret);


let tuple = [
    ("1".to_string(), "1".to_string()),
    ("2".to_string(), "2".to_string()),
    ("3".to_string(), "3".to_string()),
];
// since we used `iter()`, and data are not copyable(String), hence move semantic occurs, and we need to take the data by cloning(copy for mov data)
let ret = tuple
    .iter()
    .map(|x| x.clone())
    .collect::<HashMap<String, String>>();
println!("tupe string: {:#?}", ret);

// unlike above, since we already use `into_iter` which take ownership items we passed, to need to clone the data since ownership already passed by `into_iter`
let tuple = [
    ("1".to_string(), "1".to_string()),
    ("2".to_string(), "2".to_string()),
    ("3".to_string(), "3".to_string()),
];
let ret = tuple
    .into_iter()
    .map(|x| x)
    .collect::<HashMap<String, String>>();
println!("tupe string: {:#?}", ret);

// we can decode the tuple inside and return same structure
let tuple = [(1, 1), (2, 2), (3, 3)];
let ret = tuple
    .iter()
    .map(|x| (x.0, x.1))
    .collect::<HashMap<i32, i32>>();
println!("{:#?}", ret);

// we can also collect HashMap as Vector since it's just array of tuple-2 of keys and values
let map = HashMap::from([(1, 1), (2, 2), (3, 3)]);
let ret = map.into_iter().map(|x| x).collect::<Vec<_>>();
println!("{:#?}", ret);
```