use std::{
    collections::{HashMap, HashSet},
    vec,
};

#[allow(unused)]

fn main() {
    /// MAP
    ///
    // iterate over slice
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
    println!("mut_array.into_iter(): {:?}", array); // valid because array of copyable items
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

    /// FILTER
    ///  
    // iterate and filter element
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

    /// REDUCE
    ///
    // iterate and accumulate elements into one value
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

    let vec = vec![1, 2, 3, 4];
    let result = for i in vec {
        println!("{:?}", i)
    };

    let vec = vec![1, 2, 3, 4];
    let iter_expr = vec![1, 2, 3, 4].into_iter();
    let result = match IntoIterator::into_iter(iter_expr) {
        mut iter => 'label: loop {
            let mut next;
            match Iterator::next(&mut iter) {
                Option::Some(val) => next = val,
                Option::None => break,
            };
            let PATTERN = next;
            let () = { println!("{:?}", PATTERN) };
        },
    };
    println!("{:?}", result);
}
