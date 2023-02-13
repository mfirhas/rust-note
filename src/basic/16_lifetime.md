# Lifetime #

Lifetime adalah scope validnya suatu data ketika di-*borrowed*. Lifetime umumnya sudah implisit untuk setiap variable yang dideklarasikan. Lifetime memberi penanda pada scope dari *borrowed* value supaya compiler bisa memastikan bahwa *borrowed* value tersebut tetap valid digunakan. Lifetime harus selalu merujuk ke suatu atau beberapa *borrowed* value. Lifetime yang dirujuk harus valid selama data yang mereferensikan lifetime tersebut digunakan/valid.

Contoh deklarasi anotasi generic lifetime:
```rust
a: &'a str // variable of a with 'a lifetime on immutable data
a: &'a mut str // variable of a with 'a lifetime on mutable data
```

Metode yang digunakan Rust untuk mendeteksi validnya suatu reference disebut juga dengan *Borrow Checker*.

## Function Lifetime Annotation ##
Lifetime pada fungsi digunakan untuk memberi penanda lifetime pada parameter dan return values. Tidak bisa digunakan untuk memberi lifetime terhadap data yang di-*borrowed* dari dalam fungsi karena data tersebut akan dihapus setelah fungsi return/exit, kecuali *borrowed* value bersifat static.
Contoh deklarasi annotasi lifetime pada fungsi:
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
Pada fungsi di atas deklarasi lifetime setelah nama fungsi. Selanjutnya lifetime digunakan untuk menyambungkan lifetime parameter kepada return value. Dalam hal ini, compiler akan memilih lifetime paling pendek untuk dijadikan acuan return value. Hal itu karena kalau menggunakan lifetime terpanjang, dan ternyata return valuenya berasal dari lifetime terpendek, maka akan terjadi dangling reference. Parameter dengan lifetime terpendek itu akan di remove di scope nya yang lebih pendek sehingga jika masih ada referensi terhadap value tersebut, menjadi invalid. Inilah yang dideteksi oleh compiler rust dengan cara menambahkan lifetime annotasi.

Contoh lain ketika memberi lifetime kepada value reference di dalam fungsi:
```rust
fn fff<'a>(x: &str, y: &str) -> &'a str { // worked
    let ret = "anu";
    ret
}
fn fff2<'a>(x: &str, y: &str) -> &'a str { // doesn't work
    let ret = String::from("anu");
    ret.as_str()
}
```
Fungsi pertama berjalan karena default lifetime untuk string yang dideklarasikan secara literal adalah *static*, yaitu hidup sepanjang program berjalan. Sehingga lifetime `'a` akan menggunakan lifetime static tersebut.

Fungsi kedua tidak berjalan karena `String::from("anu")` bersifat *owned* karena data dialokasi secara dynamic sehingga akan berakhir ketika fungsi return/exit dan menyebabkan reference ke data tersebut menjadi invalid.


## Struct Lifetime Annotation ##
Lifetime pada struct berarti field-field dari struct tersebut *borrow* value dari luar. Sehingga untuk menyambung lifetime *borrowed* data tersebut ke dalam field-field struct, field tersebut harus menggunakan generic lifetime annotation.
Contoh:
```rust
fn main() {
    let string = String::from("string");
    let s = StructWithLifetime{
        string: string.as_str(),
    };
    
    println!("{:?}", s);
}

#[derive(Debug)]
struct StructWithLifetime<'a> {
    string: &'a str,
}
```
Pada contoh di atas, dilihat bahwa field `string` dari `StructWithLifetime` menggunakan lifetime dari String dari luar di dalam fungsi `main`, sehingga field tersebut valid selama `string` masih di dalam scope.

Untuk lifetime pada type lainnya seperti enum, akan sama dimana lifetime diberi anotasi setelah nama tipe.

## Method/inherent functions Lifetime Annotation ##

Lifetime juga bisa ditaruh untuk inherent functions/methods untuk mendeklarasi lifetime yang akan digunakan oleh implementor type dan fungsi/method di dalamnya.
Contoh:
```rust
// deklarasi lifetime struct yang digunakan oleh field `name`
struct Service<'a> {
    name: &'a str,
}

// deklarasi lifetime inherent items
impl<'a> Service<'a> {
    // penggunaan generic lifetime pada parameter fungsi untuk lifetime dari self
    pub fn method1(&'a self) -> &str {
        f(self.name);
        self.name
    }
}

// deklarasi lifetime inherent items, dengan lifetime Service bersifat wildcard(ignored)
impl<'a> Service<'_> {
    pub fn method2(&self) -> &str {
        self.name
    }
}

// tanpa deklarasi lifetime inherent items, dengan lifetime Service bersifat wildcard(ignored)
impl Service<'_> {
    pub fn method3(&self) -> &str {
        self.name
    }
}

// deklarasi lifetime fungsi yang dipanggil di inherent items di atas, chaining lifetime from caller to callee
fn f<'a>(s: &'a str) -> &'a str {
    println!("f() -> {}", s);
    s
}
```

## Static Lifetime(&'static) ##
Adalah lifetime dari suatu *borrowed* type yang hidup sepanjang program berjalan. Contoh yang sering kita lihat adalah ketika kita mendeklarasi constant `&str`
```rust
let static_str: &'static str = "live forever";
println!("{static_str}");
```

Contoh static lifetime yang di-infer secara otomatis oleh compiler:
```rust
const I_LIVE_LONGEST: &str = "anu";
```
Konstant `I_LIVE_LONGEST` akan bisa di-*pinjam* selama program berjalan karena *borrowed* type `str` akan di-*infered* sebagai static oleh compiler menjadi `&'static str`.
Tidak semua tipe akan di-infer secara otomatis oleh compiler sehingga butuh deklarasi static lifetime eksplisit.

## Lifetime Elision ##
Adalah set of rules yang digunakan compiler untuk menentukan lifetime suatu/beberapa *borrowed* type tanpa deklarasi eksplisit. 
Berikut 3 rules tersebut:
1. Compiler will assign unique lifetime to each parameters in function/inherent function/method.
```rust
fn foo(x: &i32) --> fn foo<'a>(x: &'a i32)
fn foo(x: &i32, y: &i32) --> fn foo<'a, 'b>(x: &'a i32, y: &'b i32)
```
2. If there's only one parameter, then the parameter lifetime will be applied to all output parameters lifetime.
```rust
fn foo<'a>(x: &i32) -> (&i32,&i32,&i32) --> fn foo<'a>(x: &'a i32) -> (&'a i32,&'a i32,&'a i32)
```
3. Apply to method with multiple input parameters and one of them is `&self` or `&mut self`, the lifetime of `self` will be applied to all output parameters.
```rust
impl<'a> Type<'a> {
    fn method(&self, &str, &str) -> (&str, &str){...}
} 
// menjadi
impl<'a> Type<'a> {
    fn method<'a, 'b, 'c>(&'a self, &'b str, &'c str) -> (&'a str, &'a str){...}
}
```
Deklarasi lifetime pada method signature tidak diperlukan ketika menulis program, karena sudah di-infer oleh lifetime elision rule yang mana lifetime yang akan digunakan adalah lifetime self dari `impl<'a>` dan `Type<'a>`.

