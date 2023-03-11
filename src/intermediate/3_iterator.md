# Iterator #

Merupakan pattern untuk melakukan perulangan pada compound/collection types tanpa control flow for/loop/while. 
Pattern ini merupakan salah satu pendekatan functional dalam melakukan looping dimana melakukan processing data pada compound/collection dengan mengkombinasikan berbagai fungsi. Kita bisa menggunakan konsep monad sebelumnya pada kali ini, yaitu monad yang merupakan kategori dengan data di dalam yang di-iterasi menggunakan functor dan arrows untuk fungsi transformasi.

Dalam wujudnya, pattern ini menyediakan 1 trait bernama `Iterator` yang merupakan bagian dari standard library yang berisi 1 type yang menjadi item iterasi, dan 1 method yang wajib diimplementasi dan sisanya merupakan methods dengan default implementations.

```rust
pub trait Iterator {

    // item to be iterated over
    type Item;

    // method to get next data in interations
    // method next use mutable reference because it needs consume the object's data and updating its data pointed to and returning previous state.
    fn next(&mut self) -> Option<Self::Item>;

    // ... rest of methods related to iterations implemented defaultly.
```

Iterator memiliki beberapa objek yang mengimplementasikan iterasi ini. Objek tersebut yang menampung data iterasi dan mengolah data menggunakan method-method pada Iterator. Diantara objek-objek tersebut adalah: `Map<I, F>`, `Iter<'a, A>`, dan lainnya yang bisa dilihat di [doc](https://doc.rust-lang.org/std/iter/trait.Iterator.html). Objek-objek tersebut tersebar di standard lib yang merupakan objek perantara iterasi dari objek yang sebenarnya memiliki data, seperti `Vec<T>`, `HashMap<K, V>`, array, slices, dan lainnya. 

## Iterator Constructor ##
Terdapat 3 method yang digunakan untuk menghasilkan objek iterator di atas, yang dipanggil oleh pemilik data, yaitu:

### `.iter()` ### 
Melakukan iterasi terhadap immutable reference(`&self`). Method ini tidak menghasilkan iterator objek yang meng-*consume* object yang di-iterasi, karena hanya di-*borrow* sebagai reference. Method ini menghasilkan objek Iterator `Iter` yang merupakan konvensi objek yang akan mengiterasi items by *immutable* *borrow*/*reference*.
```rust
let vec = vec![1,2,3,4];
let mut iterator = vec.iter();
println!("{:?}", iterator.next()); // Some(1)
println!("{:?}", iterator.next()); // Some(2)
println!("{:?}", iterator.next()); // Some(3)
println!("{:?}", iterator.next()); // Some(4)
println!("{:?}", iterator.next()); // None
```
Kenapa `iterator` mutable?, karena method `next()` akan melakukan iterasi dengan cara mengupdate underlying object yang dijadikan acuan iterasi. 
Jarang kita menggunakan method `next` secara langsung untuk melakukan proses terhadap iterasi data, biasanya kita menggunakan method yang disediakan oleh objek iterasi `Iter` itu sendiri yang mana mengimplementasi trait `Iterator` itu sendiri, seperti `map`, `filter`, `reduce`, dan lainnya. Dengan melakukan chaining methods seperti ini, maka kita tidak butuh side effects di luar iterasi karena semua *enclosed* di dalam iterator chains.


### `.iter_mut()` ### 
Melakukan iterasi terhadap mutable reference(`&mut self`). Method ini memiliki side-effects terhadap data yang di-iterasi jika memanggil method seperti `next()` dan lainnya. Method ini menghasilkan objek iterator `IterMut` yang merupakan konvensi objek yang akan mengiterasi item mutable *borrow*/*reference*. Method ini juga di chain dengan method-method lainnya yang mengambil data juga by *mutable reference*.
```rust
let mut vec = vec![1,2,3,4];
let mut iterator = vec.iter_mut();
println!("{:?}", iterator.next()); // Some(1)
println!("{:?}", iterator.next()); // Some(2)
println!("{:?}", iterator.next()); // Some(3)
println!("{:?}", iterator.next()); // Some(4)
println!("{:?}", iterator.next()); // None
```
Berbeda dengan `iter` yang melakukan borrow, `iter_mut` melakukan iterasi by mutable reference, sehingga objek yang menjadi acuan iterasi(`vec`) harus mutable.
  
### `.into_iter()` ### 
Memindahkan ownership dari data yang di-iterasi ke dalam iterator(`self`). Method ini merupakan default ketika melakukan iterasi dengan for loop.
```rust
let vec = vec![1,2,3,4];
let mut iterator = vec.into_iter();
println!("{:?}", iterator.next()); // Some(1)
println!("{:?}", iterator.next()); // Some(2)
println!("{:?}", iterator.next()); // Some(3)
println!("{:?}", iterator.next()); // Some(4)
println!("{:?}", iterator.next()); // None
// println!("{:?}", vec); // failed, because vec already moved by `.into_iter()` method invocation.
```

> Tahukah kamu! `for` loop merupakan syntatic sugar dari `IntoIterator`.
> Kode ini:
> ```rust
> let vec = vec![1, 2, 3, 4];
> let result = for i in vec {
>    println!("{:?}", i)
> };
> ```
> Sama dengan:
> ```rust
> let vec = vec![1, 2, 3, 4];
> let iter_expr = vec![1, 2, 3, 4].into_iter();
> let result = match IntoIterator::into_iter(iter_expr) {
>     mut iter => 'label: loop {
>         let mut next;
>         match Iterator::next(&mut iter) {
>             Option::Some(val) => next = val,
>             Option::None => break,
>         };
>         let PATTERN = next;
>         let () = { println!("{:?}", PATTERN) };
>     },
> };
> ```
>
> Mungkin kita bertanya, kenapa?, well, karena `for` memang membaca data compounds/collections yang implement trait `IntoIterator` secara default.
> Hal unik lainnya, extra lines yang dihasilkan tidak menambah overhead pada looping karena prinsip **Zero-cost Abstraction** dari Rust.



## Functionals ##
Secara umum, terdapat 3 fungsionalitas utama yang biasa digunakan dalam iterator:

### Map ###
*Map* adalah fungsionalitas dari iterator untuk melakukan **apply** function terhadap data yang di-iterasi. Fungsi ini digunakan untuk melakukan operasi terhadap data seperti transformasi. Map menerima 1 closure yang bertindak sebagai apply terhadap data yaitu `FnMut`.
Map pada iterator mengembalikan objek Map yang bisa juga disebut sebagai functor yang mengkombinasikan arrows.
```rust
pub trait Iterator {
    ...
    fn map<B, F>(self, f: F) -> Map<Self, F> ⓘ
        where
            Self: Sized,
            F: FnMut(Self::Item) -> B, 
    { ... }
    ...
}
```

Contoh:
```rust
fn main() {
    let slice: &[i32] = &[1, 2, 3, 4];
    let from_slice = slice.iter().map(|x| x + 1).collect::<Vec<_>>();
    println!("slice.iter(): before: {:?}", slice);
    println!("slice.iter(): after:  {:?}", from_slice);

    let mut_slice = &mut [1, 2, 3, 4];
    let from_slice = mut_slice
        .iter_mut()
        .map(|x| {
            *x += 5;
            *x + 1
        })
        .collect::<Vec<_>>();
    println!("slice.iter_mut(): before: {:?}", slice);
    println!("slice.iter_mut(): after:  {:?}", from_slice);

    let from_slice = slice.into_iter().map(|x| x * 4).collect::<Vec<_>>();
    println!("slice.into_iter(): {:?}", slice);
    println!("slice.into_iter(): {:?}", from_slice);

    println!("-------------");

    // iterate over array
    let array: [i32; 4] = [1, 2, 3, 4];
    let from_array = array.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("array.iter(): {:?}", array);
    println!("array.iter(): {:?}", from_array);

    let mut mut_array = [1, 2, 3, 4];
    let from_array = mut_array.iter_mut().map(|x| *x * *x).collect::<Vec<_>>();
    println!("mut_array.iter_mut(): {:?}", mut_array);
    println!("mut_array.iter_mut(): {:?}", from_array);

    let from_array = array.into_iter().map(|x| x * x).collect::<Vec<_>>();
    println!("mut_array.into_iter(): {:?}", array);
    println!("mut_array.into_iter(): {:?}", from_array);

    println!("-------------");

    // iterate over array_ref
    let array: &[i32; 4] = &[1, 2, 3, 4];
    let from_array = array.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("array.iter(): {:?}", array);
    println!("array.iter(): {:?}", from_array);

    let mut mut_array = [1, 2, 3, 4];
    let from_array = mut_array.iter_mut().map(|x| *x * *x).collect::<Vec<_>>();
    println!("mut_array.iter_mut(): {:?}", mut_array);
    println!("mut_array.iter_mut(): {:?}", from_array);

    let from_array = array.into_iter().map(|x| x * x).collect::<Vec<_>>();
    println!("mut_array.into_iter(): {:?}", array);
    println!("mut_array.into_iter(): {:?}", from_array);

    println!("-------------");

    // iterate over vector
    let vector = vec![1, 2, 3, 4];
    let from_vector = vector.iter().map(|x| x * x).collect::<Vec<_>>();
    println!("vector.iter(): {:?}", vector);
    println!("vector.iter(): {:?}", from_vector);

    let mut mut_vector = vec![1, 2, 3, 4];
    let from_vector = mut_vector.iter_mut().map(|x| *x * *x).collect::<Vec<_>>();
    println!("mut_vector.iter_mut(): {:?}", vector);
    println!("mut_vector.iter_mut(): {:?}", from_vector);

    let from_vector = vector.into_iter().map(|x| x * x).collect::<Vec<_>>();
    // println!("vector.into_iter(): {:?}", vector); // failed because ownership already moved into `into_iter()`.
    println!("vector.into_iter(): {:?}", from_vector);

    println!("----------");

    // iterate over hashmap
    let map = HashMap::from([(1, 2), (2, 3), (3, 4), (4, 5)]);
    let from_map = map.iter().map(|(_, v)| v * v).collect::<Vec<_>>();
    println!("map.iter(): {:?}", map);
    println!("map.iter(): {:?}", from_map);

    let mut mut_map = HashMap::from([(1, 2), (2, 3), (3, 4), (4, 5)]);
    let from_map = mut_map
        .iter_mut()
        .map(|x| {
            *x.1 += 2; // reflects to outside because it takes mutable reference to outside data.
            (*x.0, *x.1)
        })
        .collect::<HashMap<i32, i32>>();
    println!("mut_map.iter_mut(): {:?}", mut_map);
    println!("mut_map.iter_mut(): {:?}", from_map);

    let from_map = map.into_iter().map(|x| (x.0, x.1 * 4)).collect::<Vec<_>>();
    // println!("map.into_iter(): {:?}", map); // ownership moved
    println!("map.into_iter(): {:?}", from_map);

    // iterate over vector and convert it into hashset
    let vec = vec![1, 1, 2, 3, 3, 3, 5, 10];
    let set = vec.into_iter().collect::<HashSet<_>>();
    println!("{:#?}", set);

    // capturing variables from env around
    let vec = vec![1, 2, 3, 4];
    let v = 123;
    let sdf = vec.into_iter().map(|x| x * v).collect::<Vec<_>>();
    println!("---->{:?}", sdf);

    let vec = vec!["1", "2", "3", "4"];
    let v = "123";
    let sdf = vec
        .into_iter()
        .map(|x| format!("{}-{}", x, v))
        .collect::<Vec<_>>();
    println!("---->{:?}", sdf);

    let vec = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];
    let v = "123".to_string();
    let sdf = vec
        .into_iter()
        .map(|mut x| {
            x.push_str(v.as_str());
            x
        })
        .collect::<Vec<_>>();
    println!("---->{:?}", sdf);

    let vec = vec![
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
        "4".to_string(),
    ];
    let mut v = "123".to_string();
    let sdf = vec
        .into_iter()
        .map(|x| {
            v.push_str(x.as_str());
            v.clone()
        })
        .collect::<Vec<_>>();
    println!("---->{:?}", sdf);
}
```

### Filter ###
*Filter* adalah fungsionalitas iterator untuk menerapkan **predicate** terhadap data yang di-iterasi. Sehingga data yang di-ambil hanyalah data yang memenuhi predicate tersebut. Predicate adalah closure yang diterima oleh method filter yang menghasilkan *boolean*. Seperti namanya, digunakan untuk mengfilter data.
```rust
fn main() {
    ...
    fn filter<P>(self, predicate: P) -> Filter<Self, P> ⓘ
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    { ... }
    ...
}
```

Contoh:
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let filter = vec.into_iter().filter(|x| x % 2 == 0).collect::<Vec<_>>();
    println!("{:?}", filter);

    // iterate, filter, and map element
    let vec = vec![1, 2, 3, 4, 5];
    let filter = vec
        .into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect::<Vec<_>>();

    println!("{:?}", filter);

    let vec = vec!["qwe", "qwer", "wefwerwer", "lwkmeflwkef"];
    let filter = vec
        .into_iter()
        .filter(|x| x.len().lt(&5))
        .collect::<Vec<_>>();
    println!("{:?}", filter);
}
```

### Reduce ###
*Reduce* merupakan fungsionalitas yang menerapkan fungsi **accumulator** yang merangkum data di dalam iterasi menjadi 1 single data yang masih di dalam 1 kategori. Berbeda dengan fungsionalitasi2 sebelumnya yang menghasilkan collection, fungsi ini menghasilkan 1 single data hasil akumulasi dari setiap iterasi. Api ini mirip dengan Fold, bedanya fold memiliki initial value sebagai akumulator, sedangkan reduce menggunakan value pertama dalam iterasi sebagai initial value dari akumulator.
```rust
pub trait Iterator {
    ...
    fn reduce<F>(self, f: F) -> Option<Self::Item>
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    { ... }
    ...
}
```

Contoh:
```rust
fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    let reduce = vec
        .into_iter()
        .reduce(|acc, x| {
            dbg!(&acc);
            dbg!(&x);
            acc + x
        })
        .unwrap_or(0);
    println!("{:?}", reduce);

    let vec = vec![1, 2, 3, 4, 5];
    let reduce = vec.into_iter().fold(0, |acc, x| {
        dbg!(&acc);
        dbg!(&x);
        acc + x
    });
    println!("{:?}", reduce);

    let map = HashMap::from([(1, 2), (2, 3), (3, 4)]);
    let reduced_map: HashMap<i32, i32> = map
        .into_iter()
        .reduce(|acc, n| (acc.0, acc.1 + n.1))
        .and_then(|x| Some(HashMap::from([x])))
        .unwrap_or_default();
    println!("{:#?}", reduced_map);
}
```

Contoh-contoh di atas hanya contoh sederhana, untuk penggunaan yang lebih kompleks yang mungkin ditemui di dunia nyata, masih banyak API-API dari iterator yang bisa dieksplore sendiri dengan berbagai fungsi-fungsi tersendiri. Selain itu, setiap fungsi-fungsi dari iterator bisa di-*chain* satu sama lain untuk proses yang lebih kompleks.


## Why iterator? ###
Kenapa gak pake `for` loop biasa aja kaya di Golang?, Hal ini ada kaitannya dengan kecenderungan Rust untuk lebih Functional ketimbang Procedural. `for` loop merupakan construct procedural, yang bersifat imperative, dilihat dari cara penggunaannya, yang memproses instruksi 1 per 1, dan melakukan increment terhadap index seperti yang umum di bahasa lain seperti `for (int i=0; i<10; i++)`. Rust secara idiom lebih cenderung ke FP yaitu menerapkan pattern iterator dengan dibarengi konsep monad yang mana fungsi-fungsi dikomposisi sedemikian rupa untuk memproses semua isi data collection. Cara iterator lebih declarative dilihat dari cara ngodingnya, deklarasi behaviournya, dan bagaimana objek dan fungsi-fungsi dikomposisi, selain itu juga terdapat sifat lazy dari setiap fungsi yang dikomposisi.

Iterator juga cara untuk mengurangi basis path dari program kita, sehingga bisa mengurangi liabilitas ketika menulis program.

Melihat dari [benchmark](https://doc.rust-lang.org/stable/book/ch13-04-performance.html#comparing-performance-loops-vs-iterators), terlihat bagaimana **Zero-cost Abstraction** pada Rust membuat semua abstraksi iterator memiliki performa yang sama dengan `for` loop biasa, *bahkan lebih cepat!*