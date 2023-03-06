# Functional Programming #

*Say this with me:*
> *Monad is just a monoid in the category of endofunctors, What's the problem?* [[1]](https://james-iry.blogspot.com/2009/05/brief-incomplete-and-mostly-wrong.html)

*Before jumping into functional programming world, you have to be able to decipher what above sentence is all about. Because functional programming mostly revolves around Monad concept. But, before we decipher that, I will explain some basic concepts related to functional programming first.*

Functional Programming merupakan pendekatan lain di dalam dunia pemograman(selain Procedural Programming) dimana kode program dirangkai sedemikian rupa menggunakan beberapa konsep dan notasi matematika seperti *set* dan *category*.

Berbeda dengan Procedural Programming yang terinspirasi langsung dari Turing Machine(Alan Turing), Functional Programming datang dari Church's Thesis(Alonzo Church) dimana program melakukan komputasi menggunakan serangkaian fungsi-fungsi yang di-*compose* sedemikian rupa. Church Thesis juga berpendapat bahwa setiap fungsi yang computable harus turing complete(computable by Turing Machine), sehingga Church's Thesis ini juga bisa disebut sebagai abstraksi lebih besar dari Turing Machine tentang bagaimana meng-ekspresikan komputasi ke dalam notasi matematika(fungsi).[[1]](https://en.wikipedia.org/wiki/Church%27s_thesis_(constructive_mathematics)),[[2]](https://en.wikipedia.org/wiki/Church%E2%80%93Turing_thesis)

Berikut beberapa perbedaan Procedural Programming dan Functional Programming:

| Procedural                              | Functional                                    |
|-----------------------------------------|-----------------------------------------------|
| Imperative                              | Declarative                                   |
| Mutability                              | Immutability                                  |
| Stateful                                | Stateless                                     |
| Statement                               | Expression                                    |
| Top-Down                                | Left-Right                                    |
| Non-Algebraic                           | Algebraic                                     |
| Low-context                             | High-context                                  |
| E.g. Assembly, C, Go                    | E.g. Haskell, OCaml, Rust                     |

- Imperative cenderung menerapkan code program *step-by-step* dari atas ke bawah dan melakukan mutasi terhadap suatu data(dynamic). Sedangkan Declarative cenderung merangkai code program dalam bentuk *function compositions* dengan transisi data di antaranya tanpa ada mutasi terhadap data original.
- Statements cenderung tidak memiliki values, hanya command yang melakukan proses terhadap suatu data yang dimasukkan. Sedangkan Expressions merupakan values itu sendiri yang bisa dirangkai dengan fungsi-fungsi lainnya untuk proses komputasi.
- Pada bahasa yang memiliki non-algebraic data types, tipe data cenderung refleksi langsung dari layout memory secara low level. Tipe data tersebut tidak bisa/kurang optimal jika dikorelasikan dengan teori set dan kategori pada aljabar. Contoh: Pointer(C, C++, Go, etc), class/object(Java, C#, etc), interface(Go, Java, C, etc), beberapa tipe data pada bahasa C dan C++ dimana jika tidak dideklarasikan value, maka akan memberi default value diluar dugaan kita dan bisa menghasilkan UB(Undefined Behaviour).
- Pada Bahasa yang memiliki algebraic data types, tipe data cenderung refleksi dari teori set dan kategori di dalam aljabar, sehingga sangat bisa dinotasikan dengan konsep tersebut di dalam pemograman. Contoh: Product Types(record, struct, tuple), Sum Types(enum in Rust, etc), reference types, hampir semua tipe data di dalam Rust bersifat algebraic karena mereka semua antara *set* dan *kategori* di dalam aljabar. Oleh karenanya di dalam Rust tidak ada empty/default values(kecuali `unsafe`), jika mau, maka harus dideklarasi secara manual. Trait di dalam rust pun bersifat algebraic karena berbeda dengan interface pada bahasa lain, trait bisa digunakan sebagai superset dari suatu tipe, untuk memberi spesifikasi dan juga mengelompokkan suatu tipe data/set.
- Bahasa procedural cenderung menghandle error menggunakan throws/try-catch, sedangkan bahasa functional cenderung menggunakan sum type.

Melihat beberapa perbedaan di atas, kita bisa lebih lanjut menjabarkan beberapa karakteristik dari Functional Programming, di antaranya:
- *Immutability*: sifat immutability merupakan requirement dasar dan wajib dalam functional programming, karena di dalam FP, side effects dan segala macam variants dari suatu data sangat dihindari, karena bisa merusak abstraksi dan komposisi data dan fungsi. Ketika ingin melakukan transformasi/perubahan data, yang biasa dilakukan adalah menerapkan fungsi transformasi pada data original tanpa mengubah data original tersebut.
- *First-order function*: merupakan function yang mana function diterapkan sebagai unit dasar dalam komputasi yang bisa dilempar kemana-mana sebagai parameter pada fungsi lainnya atau sebagai value/argument.
- *Higher-order function*: merupakan function yang meng-komposisi beberapa fungsi untuk melakukan komputasi dan menghasilkan suatu value atau function. Kadang disebut juga sebagai kombinator, karena fungsinya yang mengkombinasikan beberapa *first-order functions*.
- *Algebraic types*: merupakan tipe data yang bisa merepresentasikan konsep set dan kategori yang ada pada aljabar kedalam kode pemograman dan menerapakan beberapa fungsi padanya.
- *Monadic*: merupakan tipe data yang memiliki sifat monadic, yaitu monoid yang di-wrap ke dalam type constructor dengan beberapa fungsi yang bisa meng-*flatten* data di dalamnya dan me-*mapping* data tersebut dengan suatu/beberapa proses(arrows).

*Disclaimer*: perbedaan dan konsep di atas tidak selalu bersifat absolute. Ada suatu bahasa pemograman yang cenderung procedural, ada yang cenderung functional, dan ada juga yang pure. Contoh: Assembly purely procedural, C kebanyakan procedural, Go kebanyakan procedural, Rust kebanyakan functional, Haskell purely functional.


# Konsep Dasar #

Functional programming revolve around istilah matematika aljabar, khususnya dalam ranah ilmu set dan kategori. Berikut beberapa hal yang berkaitan dengan Functional Programming:
## Set ##
Set adalah sekumpulan objek yang memiliki kesamaan dan unik.
 Unik dalam artian tidak boleh ada objek yang sama/ganda di dalam set tersebut.
Di dalam dunia pemograman, **set** bisa disamakan dengan tipe data, e.g integer, float, chars, booleans, etc.

## Category ##
Category merupakan abstraksi lebih besar dari set, dimana merupakan sekumpulan dari objek di dalam suatu set. Bisa juga disebut sebagai abstraksi suatu data di dalam suatu set di dalam suatu category. Selain itu objek yang telah menjadi category ini memiliki 2 karakteristik:
- Memiliki arrows function ter-*associate* dengannya. Arrows function merupakan semacam anonymous function yang dapat di-embed menjadi argument untuk melakukan transformasi category.
- Memiliki identity arrows, merupakan identity function yang menerima data dan mengembalikan data yang sama, `f(x) -> x, x ∈ A`

Di dalam dunia pemograman, **category** bisa disamakan dengan *container*/*type constructor*, yaitu tipe data yang memiliki data lain di dalamnya, contoh: Vec\<T>, HashMap<K,V>, LinkedList\<T>, etc. Setiap container tersebut memiliki associative functions yang memproses arrows yang diterima.

Category bisa juga disebut sebagai *shape*, dimana, sekalipun memiliki tipe data yang berbeda programmatically, jika memiliki shape yang sama, maka masih bisa dimasukkan ke kategori yang sama. 

Contoh: array dari 2-tuple([(k1,v1), (k2,v2), ...]) dan HashMap<K,V> memiliki tipe data yang berbeda, akan tetapi memiliki kategori yang sama, karena sejatinya map itu memang sekumpulan/array dari key dan value.

## Magma ##
Magma adalah suatu *set* yang berisi data dengan elemen yang sama, ketika operasi binary diaplikasikan ke dua buah data di dalam set tersebut, menghasilkan data yang juga berada di dalam set tersebut. 

> a ∈ S, b ∈ S --> a·b ∈ S

Contoh adalah operasi pada bilangan bulat positif.
Operasi binary yang bisa di-aplikasikan terhadap bilangan bulat positif adalah +,*,-,/.

`a ∈ UInt, b ∈ UInt --> a·b ∈ UInt`

a·b ∈ UInt:
- a+b ∈ UInt, 1+2   = 3, 3 ∈ UInt
- a\*b ∈ UInt, 1\*2 = 2, 2 ∈ UInt
- a/b ∈ UInt, 1/2   = 0.5, 0.5 ~∈ UInt (not magma)
- a-b ∈ UInt, 1-2   = -1, -1 ~∈ UInt (not magma)

Sehingga yang termasuk magma untuk UInt adalah `a+b` dan `a\*b` karena hasil nya pasti di dalam set yang sama yaitu Unsigned Integer.


## Semigroup (Magma + Associative) ##
Semigroup adalah suatu set yang memiliki operasi binary terhadap lebih dari dua input data dan bisa dikombinasikan dengan berbagai cara sehingga tetap menghasilkan hasil yang sama di dalam set yang sama.
> a ∈ S, b ∈ S, c ∈ S --> (a·b)·c = a·(b·c) ∈ S

Contoh:
```
(1+2)+3 = 1+(2+3)   = 6 
3+3     = 1 + 5     = 6
```
Maka operasi `+` terhadap 3 bilang bulat di atas bersifat semigroup karena bagaimanapun urutan operasi yang dilakukan, akan menghasilkan nilai yang sama.

## Monoid (Magma + Associative + Identity) ##
Monoid adalah suatu set yang memenuhi kriteria:
- Magma yang bersifat associative untuk data lebih dari 2 -> Semigroup, dan
- Semigroup yang memiliki identity element/function sehingga element/function ini tidak merubah data/value tersebut(neutral)
  Identity element bisa juga disebut debagai default values/empty values dari suatu set jika diterapkan suatu operasi binary tidak mengubah value dari data di dalam set tersebut, contoh operasi pertambahan, maka default values dari integer adalah `0`.

Setiap set memiliki identity element(jika value) atau identity function(jika function) yang mana apabila diapply dengan suatu operasi binary, akan menghasilkan value yang sama. Hampir di semua bahasa pemograman dengan tipe data tertentu memiliki identity element atau identity function yang bisa diterapkan padanya. 
Hampir semua tipe data di Rust bisa disebut monoid.

## Arrow ##
Merupakan function yang akan di-inject ke dalam functor dan terkomposisi melakukan pemosresan data dari sebuah category.

```
(A) -> B
```
Merupakan arrow yang memetakan category A menjadi B.

## Functor ##
Functor adalah proses mapping antara suatu category ke dalam category lainnya.
Functor memiliki beberapa karakteristik:
- Preserve identity arrows(functions passed into the functors).
- Preserve the composition of the arrows processing the elements inside the catgory into another category(codomain).

```
F(X) -> Y

F(X) -> F(Y)
```

Jadi functor adalah fungsi atau tipe data yang memiliki fungsi-fungsi yang dapat meng-komposisikan berbagai arrows(lambda) untuk memproses setiap elements di dalam category tersebut.(*mindblown!*)

## Endofunctor ##
Sama dengan functor, hanya saja endofunctor memetakan ke category yang sama, sekalipun object/set di dalamnya berubah.

*Akhirnya kita sampai juga pada apa itu monad*.

## Monad ##
Jika kita kembali pada quote di atas, *"Monad is just a monoid in the category of endofunctors"*, 

maka kita dapat menyimpulkan monad sebagai ***category dari sebuah monoid yang memiliki beberapa endofunctors yang mengkomposisi arrows***.

Atau jika ditulis dalam bentuk lain: 

> *Monad = Category\<Monoid>.Endofunctors(arrows)*


# Konsep Lanjutan #
Berikut kita akan membahas sesuatu yang lebih konkrit dari teori di atas yang mana dapat diterapkan dibanyak bahasa pemograman yang memiliki fitur functional.

## Immutability ##
Immutability merupakan konsep dari dunia matematika dimana objek di dalam matematika bersifat immutable. Tidak ada state di dalam matematika, perubahan data, dari suatu variable, atau yang biasa disebut dengan stateful.

Konsep dari immutability itu sederhana, suatu data yang telah dibentuk tidak dapat diubah selama pemosresan, jikapun diubah, biasanya secara monadic, dimana original data di-preserved, dan menghasilkan data baru yang berasal dari data original.

## Pure Function ##
Pure Function merupakan fungsi yang tidak merubah data apapun yang di-capture dari luar. Pure function ini dalam bahasa pemograman merupakan closure yang tidak meng-capture mutable data sekitar. Yang diharapkan dari pure function ini adalah tidak ada side-effect yang dihasilkan oleh fungsi tersebut. Side effect ini bisa berupa data yang diubah2 dan stateful sepanjang program berjalan.

## First-order Function ##
Merupakan fungsi yang bisa dijadikan sebagai value/argument.
First-order function belum tentu pure function, dan sebaliknya. Sedangkan di dalam functional programming, dibutuhkan dua hal ini.
First-order function merupakan arrows pada konsep di atas, yang bisa menjadi fungsi yang melakukan `morphism` pada functors.

## Higher-order Function ##
Merupakan fungsi yang meng-kombinasikan satu atau beberapa first-order function dan menghasilkan value atau fungsi lainnya.
Bisa juga disebut sebagai kombinator. Higher-order function bisa dikategorikan ke dalam functor karena memiliki kombinasi fungsi di dalamnya.

## Referential Transparency ##
Merupakan konsep dimana setiap value di dalam kode pemograman bisa di-replace dengan expression yang menghasilkan value yang sama, tanpa mengubah behaviour program. 
Hal ini berguna untuk maintainability sebuah program, dan juga sebagai cara untuk mengembangkan program lebih lanjut dengan mudah tanpa refactor terlalu besar.
Salah satu requirement dalam referential transparency ini adalah immutability, dimana value dan expression yang digunakan harus bebas dari side-effects, karena jika ada side-effects, maka akan mengganggu maintainability.