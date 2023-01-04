# Collection Types #

Compound dan Collection types hampir mirip kecuali Collection type lebih dinamis dan disimpan ke dalam heap memory.

## Vector `Vec<T>` ##
Vector merupakan tipe data seperti list. Perbedaan vector dengan array dan slice adalah vector memiliki ownership terhadap data yang dikandung baik itu tipe data yang bersifat *copyable*(stack-allocated) atau *clonable*(heap-allocated). Hal ini menjadikan vector akan invalid ketika berpindah ownership atau keluar dari scope.
Vector memiliki tipe berparameter(*generic type*) `<T>` yang merupakan tipe data dari elemen-elemen terkandung.
Contoh:
```rust
let mut vec_string: Vec<String> = Vec::new();
vec_string.push("first string".to_string());
vec_string.push("second string".to_string());
vec_string.push("third string".to_string());
vec_string.push("fourth string".to_string());
println!("{}", vec_string); // prints ["first string", "second string", "third string", "fourth string"]

let mut vec_i32 = vec![1,2,3,4];
vec_i32.push(5)
println!("{:?}", vec_i32); // [1, 2, 3, 4, 5]
vec_i32.remove(1); // remove element by index and shift elements after it to the left.
println!("{:?}", vec_i32); // [1, 3, 4, 5]
```
Pada contoh di atas dapat dilihat terdapat 2 jenis deklarasi vector, yaitu dengan menggunakan inisialisasi `Vec::new()` atau dengan menggunakan macro `vec!`. Ketika menggunakan macro, tipe data di-*infer* secara otomatis oleh compiler. Terlihat bahwa vector harus bersifat *mutable* agar bisa di ubah(assign/update).

## HashMap `HashMap<K, V, S = RandomState>` ##
Tipe data map yang memetekan suatu key(K) ke suatu value(V). Berbeda dengan tipe data lainnya yang sudah termasuk ke dalam *prelude*, hashmap harus diimport terlebih dahulu sebelum digunakan.
```rust
use std::collections::HashMap;

fn main() {
    let mut map: HashMap<i32, &str> = HashMap::new();
    map.insert(1, "test");
    println!("{map:?}"); // {1: "test"}
    map.insert(1, "updated_value"); // if key 1 already existed, then the value will be updated, else will be inserted
    map.insert(2, "test2");
    map.insert(3, "test3");
    map.insert(4, "test4");
    println!("{map:?}"); // {2: "test2", 1: "updated_value", 3: "test3", 4: "test4"}
    map.remove(&2); 
    println!("{map:?}"); // {1: "updated_value", 4: "test4", 3: "test3"}
    let key_1 = map.get(&1); // `get` will read the value by key's reference returning Option<V>. If not exist will return `None`.
    println!("{:?}", key_1); // Some("updated_value")

    let mut new_map: HashMap<&str, &str> = HashMap::from(
        [
            ("key_1","value_1"),
            ("key_2","value_2"),
            ("key_3","value_3"),
            ("key_4","value_4"),
        ]
    ); // initialize map with a known keys and values using array of tuples (K,V)
    println!("{new_map:?}") // {"key_3": "value_3", "key_4": "value_4", "key_2": "value_2", "key_1": "value_1"}
}
```