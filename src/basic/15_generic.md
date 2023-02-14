# Generic #

Generic merupakan polimorfisme dari suatu tipe data. Rust menerapkan *Monomorphization* untuk generic type, yaitu dengan membuat setiap copy dari tipe data yang dideklarasikan pada saat compile time. Hal ini membuat generic pada Rust *fully zero-cost*, yang berbeda dengan generic pada Java dan Go dimana Java menerapkan *Type-erasure* dan golang menerapkan *Stenciling* yang keduanya masih menyisakan setidaknya sedikit runtime overhead.
Generic dapat diterapkan pada tempat seperti: `functions`, `type aliases`, `structs`, `enumerations`, `unions`, `traits` and `implementations`.

Berikut beberapa penerapan Generic pada Rust:

## Function generic parameters ##
Generic parameter pada deklarasi fungsi untuk memberikan parameter bertipe untuk parameter fungsi/method.
```rust
fn function<T>(param: T) -> T {
    ...
}
```

Ketika ingin memanggil fungsi generic tersebut:
```rust
function::<i32>(123); // secara eksplisit mendefinisikan tipe data yang diinginkan
let t = function(123); // secara implicit di-infer dari tipe data param, 123 -> i32(default value of number type)
```

Fungsi dengan *dedicated* type akan di-generate sehingga tidak ada lagi *runtime overhead* ketika program berjalan.


## Struct generic fields ##
Generic untuk tipe data pada field-field di dalam struct.
```rust
struct A<T> {
    a: T,
    b: T,
} // single generic param

struct A2<T, E> {
    a: T,
    b: E.
} // multiple generic params
```

Ketika ingin memanggil struct generic tersebut:
```rust
let a = A::<f32> {
        a: 123.2,
        b: 23.0,
    }; // secara eksplicit mendefinisikan tipe data yang diinginkan

let a = A {
        a: 123,
        b: 34,
    }; // secara implicit di-infer dari tipe data yang dimasukkan ke field2 struct -> i32

let a2 = A2::<i32,f32> {
        a: 123,
        b: 34.4,
    }; // secara eksplicit mendefinisikan tipe data yang diinginkan

let a2 = A2 {
    a: 123,
    b: 34.5,
}; // secara implicit di-infer dari tipe data yang dimasukkan ke field2 struct -> i32 dan f32
```


## Enum generic associated data ##
Generic untuk tipe data yang berkaitan dengan deklarasi enum.
```rust
enum B<T> {
    Field1,
    Field2(T),
    Field3 {
        t: T,
    }
} // single generic param

enum B2<T, E> {
    Field1,
    Field2(T),
    Field3 {
        t: E,
    }
} // multiple generic params
```

Ketika ingin memanggil enum generic tersebut:
```rust
let b = B::<i32>::Field2(123); // secara eksplicit mendefinisikan tipe data yang diinginkan
let b = B::Field2(123); // secara implicit mendefinisikan tipe data yang diinginkan

let b2 = B2::<i32, f32>::Field2(123); // secara eksplicit mendefinisikan tipe data yang diinginkan
let b2: B2<&str, i32> = B2::Field2("123"); // deklarasi multiple generic params untuk enum harus eksplicit karena hanya 1 value enum dari multiple enum yang dideklarasikan yang digunakan, sehingga tipe harus eksplicit untuk value lainnya yang tidak/belum dipanggil.
```

## Methods generic parameters ##
Generic untuk tipe data pada penerapan *associated functions* pada suatu tipe.
```rust
struct A<T> {
    a: T,
    b: T,
}

impl<T> A<T> {
    fn method1(param: T) -> T {
        param
    }

    fn method2(param: T) {
        ...
    }
}
```

Ketika ingin memanggil generic methods tersebut
```rust
let a = A::<i32> {
        a: 123,
        b: 234,
    };
a.method2(123);
let a = A::<i32>::method1(123);
```

## Trait with generic parameter ##
```rust
pub trait MyTrait<T> {
    fn method1(param: T) -> T;
}

pub struct MyStruct<T> {
    field1: T,
}

// ketika ingin mengimplementasikan suatu trait dengan generic parameter, tipe konkrit harus di deklarasi.
impl MyTrait<String> for MyStruct<T> {
    fn method1(param: String) -> String {
        String::from("hello")
    }
}
```

## Type alias with generic ##
```rust
type MyResult<T> = Result<T, String>
```
Sehingga cukup menggunakan `MyResult` ketika ingin mengembalikan *fallible operation* dengan error tipe string.