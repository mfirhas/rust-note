// lifetime subtyping
pub fn lifetime_subtyping() {
    // 'b
    let b = String::from("lifetime b"); // lifetime of b is 'b
    let c;
    let d;
    {
        // 'a
        let a = String::from("lifetime a"); // lifetime of a is 'a
        c = lifetime_b(&a, &b);
        /// error, because c takes lifetime of a which is the current lifetime, while c has type &'b str, and 'a is not subtype of 'b, instead 'b is subtype of 'a
        /// because b live longer than a, b outlives a.
        // c = lifetime_a(&a, &b);
        println!("{c}");

        // d get value of lifetime 'a, which is the current lifetime.
        // while lifetime of d declared above is 'b.
        // this works because lifetime is covariant each other.
        // means that lifetime can be substituted with its variances.
        // d is declared inside lifetime 'b, and 'b is subtype of 'a, hence 'a and 'b have relations eachother that 'b extends 'a lifetime.
        // based on this, d will have shorter lifetime than it was declared above, hence cannot live beyond this scope.
        d = lifetime_a(&a, &b);
        println!("{d}");
    }
    /// can still access c, because c lifetime is 'b, and value of c returned from lifetime_b() takes lifetime of b, and b is beyond a.
    println!("{c}");

    // ! will error because d lifetime has been shortened to 'a inside above scope.
    // println!("{d}");
}

// 'b: 'a means b lifetime outlives a, means b live longer than a, means b should live at least as a, means a must die first then b.
fn lifetime_b<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'b str {
    b
}

fn lifetime_a<'a, 'b: 'a>(a: &'a str, b: &'b str) -> &'a str {
    a
}
