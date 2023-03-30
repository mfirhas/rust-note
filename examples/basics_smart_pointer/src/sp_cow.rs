use std::borrow::Cow;

pub fn do_cow() {
    let literal = "anu";
    let mut new_cow = Cow::from(literal);
    let reference_new_cow = &new_cow;
    println!("{:?}", reference_new_cow);
    let dereference_new_cow = &*new_cow;
    println!("{:?}", dereference_new_cow);

    // if values already in owned form, no cloning happen. Otherwise convert it to owned form by cloning the data into heap.
    // cloning happen because slice_cow is in borrowed form &str.
    let mut_string = new_cow.to_mut();
    let owned = new_cow.into_owned();

    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let mut slice_cow = Cow::from(slice);

    println!("{:?}", slice_cow);
    let reference_slice_cow: &[i32] = &slice_cow;
    println!("{:?}", reference_slice_cow);
    let dereference_slice_cow = &*slice_cow;
    println!("{:?}", dereference_slice_cow);

    // if values already in owned form, no cloning happen. Otherwise convert it to owned form by cloning the data into heap.
    // cloning happen because slice_cow is in borrowed form &[i32].
    let mut_string = slice_cow.to_mut();
    let owned = slice_cow.into_owned();

    let v = vec![1, 2, 3, 4, 5];
    let mut cow_v = Cow::from(v);

    match cow_v {
        Cow::Borrowed(x) => println!("{:?} is borrowed", x),
        Cow::Owned(ref x) => println!("{:?} is owned", x),
    }

    // no clone happen because v already in owned form.
    let mut_vec = cow_v.to_mut();
    let owned_vec = cow_v.into_owned();

    let command = "this should be all upperCase";
    println!("{:?}", check_uppercase(command));

    let v = vec![1, -2, 3, -4, -5];
    let input = Cow::from(v);
    // no cloning happen, because v already in owned form.
    println!("{:?}", check_vector(input));

    let v = &[1, -2, 3, -4, -5] as &[i32];
    let input = Cow::from(v);
    // cloning happen because v is in borrowed form &[i32].
    println!("{:?}", check_vector(input));
}

// this function return either borrowed &str or owned String
fn check_uppercase(s: &str) -> Cow<str> {
    if s.chars().all(|x| x.is_uppercase()) {
        return s.into();
    }

    s.to_uppercase().into()
}

fn check_vector(v: Cow<[i32]>) -> Vec<i32> {
    // if v already in owned form, no cloning happen, otherwise clone it and take ownership
    let it = v.into_owned().into_iter(); // if v is owned form, this line is zero cost, otherwise costing it cloning into heap.
    let ret = it.filter(|x| x.is_positive()).collect::<Vec<i32>>();
    ret
}
