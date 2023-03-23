# Algebraic Types #

Algebraic Types merupakan tipe data gabungan(*compound*) yang dibentuk dari kombinasi notasi matematika berupa: *AND* dan *OR*.
Rust memiliki 2 jenis *Algebraic Types* yaitu: *Product Type* dan *Sum Type*.

## Product Type ##
Tipe ini merupakan tipe data yang bersifat kombinatorik terhadap semua kemungkinan value-value yang ada di dalamnya. Tipe ini menggunakan logika *AND* untuk membentuknya. Seperti logika *AND* pada umumnya, semua harus ada atau tidak sama sekali. Hal ini karena jika ada satu value kosong, maka kombinasi dari semua nya menjadi hilang. Kita bisa manganalogikan *Product* sebagai hasil dari sebuah perkalian, dimana jika ada 1 value saja yang 0, maka semua jadi kosong. Jadi ini adalah tipe yang mengharuskan kita memberi value kepada semua field yang ada dalam tipe ini.

Terdapat 2 jenis Product Type di Rust, yaitu: `struct` dan `tuple`. Ketika hendak mengdeklarasikan 2 hal tersebut kita diharuskan mengisi semua field yang ada padanya atau kalau tidak mendapat error.

### Struct ###
Merupakan tipe bentukan yang terdiri dari beberapa fields dengan values dari berbagai macam tipe data.
Deklarasi:
```rust
struct Volume {
    height: u8,
    width: u8,
    length: u8,
}
```

Inisialisasi:
```rust
let volume = Volume {
    height: 12,
    width: 10,
    length: 15,
}
```
Ketika inisialisasi ketiga field harus diisi atau akan error.
Manakah sifat kombinatorik yang dimaksud?, bisa kita lihat pada contoh di atas, setiap field punya tipe u8, dengan begitu ketika dikombinasikan dengan logika *AND* atau dimultiplikasikan memiliki total kombinasi product yaitu: 256 * 256 * 256 = 16777216 kemungkinan representasi dari tipe `Volume`.

### Struct with Tuple ###
Deklarasi struct tanpa fields name. Merupakan gabungan struct dan Tuple.
```rust
#[derive(Debug)]
struct Volume(u8,u8,u8);

let volume = Volume(5,10,20);
println!("{:?}",volume);
println!("{}",volume.0);
println!("{}",volume.1);
println!("{}",volume.2);
```

## Tuple ##
Merupakan tipe bentukan yang terdiri dari values dari berbagai macam tipe data tanpa deklarasi field.
Contoh:
```rust
let tuple_1 = (1, true, 'a');
println!("{}",tuple_1.0); // print 1
println!("{}",tuple_1.1); // print true
println!("{}",tuple_1.2); // print 'a'

let tuple_2: (i32,bool,char) = (1, true, 'a');
println!("{}",tuple_2.0); // print 1
println!("{}",tuple_2.1); // print true
println!("{}",tuple_2.2); // print 'a'

let (tuple_elem_1, tuple_elem_2, tuple_elem_3) = (1, true, 'a');
println!("{}",tuple_elem_1); // print 1
println!("{}",tuple_elem_2); // print true
println!("{}",tuple_elem_3); // print 'a'
```

## Sum Type ##
Tipe ini kebalikan dari Product Type. Tipe ini menggunakan logika *OR* dimana cukup 1 value yang dibutuhkan dari semua kemungkinan value. Seperti namanya *Sum* yang mirip operasi penjumlahan jika ada 1 saja non-null value maka masih akan mendapatkan hasil non-null, e.g. 1+0+0+0 = 1. 

Rust memiliki Sum Type berupa `enum`. Berikut contoh penggunaan enum:
```rust
enum Day {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

let day = Day::Fri;
```

Seperti contoh di atas, terdapat 7 hari yang ada, akan tetapi value yang dibutuhkan hanya 1 yaitu salah satu dari value-value yang ada.
Di Rust terdapat 2 *Sum Type* yang sering digunakan yaitu: *Result* dan *Option*. 
*Result* digunakan untuk operasi-operasi yang mempunyai kemungkinan error.
```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
*Option* digunakan untuk operasi-operasi yang mempunyai kemungkanan null.
```rust
pub enum Option<T> {
    None,
    Some(T),
}
```

Contoh penggunaan:
```rust
let result: Result<String, String> = Ok("Success".to_string());
println!("{}", result.unwrap()); // Print "Success"
let result: Result<String, String> = Err("Failed".to_string());
println!("{}", result.unwrap_err()); // Print "Failed"
```


## Kombinasi Product Type dan Sum Type ##
Kita bisa mengkombinasikan 2 tipe ini untuk menciptakan model data yang lebih kompleks. Bisa dengan memasukkan *Product Type* ke dalam *Sum Type*, atau sebaliknya. Salah satu contoh yang sering digunakan adalah ketika kita ingin menerima data json yang memiliki kemungkinan beberapa fields nya kosong/null. 

Berikut contohnya:
```rust
struct RequestJson {
    id: i64,
    name: String,
    phone: Option<String>,
}

let request = RequestJson {
    id: 123,
    name: "anu".to_string(),
    phone: None, // if from json payload this field `null`, then None is given to us.
}
```
Contoh di atas menunjukkan contoh data yang menerima request json dimana field `phone` bisa `null` dari json payload.

```rust
enum Shape {
    Rectangle2DTuple(i32,i32),
    Rectangle3DTuple(i32,i32,i32),
    Rectangle2DStruct{
        length: i32,
        width: i32,
    },
    Rectangle3DStruct{
        length: i32,
        width: i32,
        height: i32,
    },
}

let shape = Shape::Rectangle2DTuple(10,20);
let shape = Shape::Rectangle3DStruct{
    length: 12,
    width:13,
    height:14,
};
```

Pada contoh di atas kita melihat kombinasi product type dan sum type. Tipe enum `Shape` memiliki value-value yang dikombinasikan dengan tuple(Rectangle2DTuple & Rectangle3DTuple) dan struct(Rectangle2DStruct & Rectangle3DStruct). Masing-masing varian enum tersebut harus mengisi semua value untuk tuple dan struct masing-masing secara utuh atau deklarasi enum gagal.

## Pattern Matching ##
Bahasa yang memiliki *Algebraic Types* selayaknya juga memiliki fitur *Pattern Matching*. Hal ini digunakan untuk melakukan pengecekkan tipe, struktur, dan value-value yang terkandung di dalam *Algebraic Types* tersebut. Rust memiliki feature ini menggunakan `match`. Setiap enumerasi tipe/value dari match disebut `arm`.
Pattern matching bersifat ***exhaustive*** yaitu semua kemungkinan signature/value harus dienumerasi ke-dalam setiap *arms* dari value tersebut.
Jika suatu value dengan tipe tertentu memiliki banyak enumerasi value yang ada padanya, tentunya tidak mungkin ditulis semua, selain itu pula kadang kita hanya butuh mengecek sebagian saja dari kemungkinan value yang ada. Untuk ini kita bisa mengabaikan value/signature lain dengan menggunakan *wildcard* untuk sisa dari value/signature yang tidak kita deklarasikan.

Contoh:
```rust
enum Gender {
    Male
    Female
}

let gender = Gender::Female;
match gender {
    Gender::Male => println!("Gender is male"),
    Gender::Female => println!("Gender is female"),
}
```

Contoh dengan wildcard:
```rust
let num: u8 = 12;
match num {
    1 => println!("one"),
    _ => panic!("gone")
}
```
Pada contoh di atas ada kemungkinan 255 value dari num(unsigned 8 bit integer), ketika kita cuma butuh mengecek beberapa kemungkinan value saja dari variable dalam pattern matching, kita cukup menambahkan `_` pada akhir *arm* dari pattern matching sehingga kita bisa mangabaikan sisa dari semua kemungkinan enumerasi.


<span style="color:red">***NOTE Wildcard selalu berada di akhir karena sifat exhaustive pattern matching dari atas ke bawah, sehingga jika kita tidak meletakkan wildcard paling bawah, maka value apapun akan masuk ke dalam *arm* wildcard sehingga menghasilkan *unintended* consequence.***</span>

`match` juga sering digunakan untuk mengecek value dari 2 tipe enum `Result` dan `Option`, serta men-destrukturalisasi tipe-tipe tersebut dan mengambil value di dalamnya.
Ketika suatu operasi memungkinkan mengembalikan value kosong, maka tipe data yang cocok digunakan adalah `Option` dengan melakukan pengecekkan:
```rust
let optional_value = Some("thing");
match optional_value {
    Some(thing) => println!("there is a {}", thing), // print "there is a thing"
    None => println!("there is nothing"),
}
// akan return "there is a thing"
```

Ketika suatu operasi memungkinkan mengembalikan error, maka tipe data yang cocok digunakan adalah `Result` dengan melakukan pengecekkan:
```rust
let error_value = Err("error");
match error_value {
    Ok(result) => println!("Success with result {}", result),
    Err(error) => println!("failed with error {}",error) // print "failed with error error"
}
// akan return "failed with error error"
```