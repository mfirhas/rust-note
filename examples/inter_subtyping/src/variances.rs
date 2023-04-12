// source: https://stackoverflow.com/questions/55344893/what-is-an-example-of-contravariant-use-in-rust

// contravariances
struct MyContraType<Mixed> {
    k1: fn(Mixed), // contravariant over Mixed
}

// Rust is contravariant over function parameters
fn contra_example<'short>(
    mut a: MyContraType<&'short u8>,
    mut b: MyContraType<&'static u8>,
    x: fn(&'short u8),
    y: fn(&'static u8),
) {
    // a = MyContraType<short>.fn(short)
    // b = MyContraType<static>.fn(static)
    // x = fn(short)
    // y = fn(static)
    // static: short
    // short is supertype
    // static is subtype
    // static <: short

    // x(short)  passed into a.k1(short)  = bivariant,     passed
    a.k1 = x;

    // y(static) passed into a.k1(short)  = covariant,     failed
    // because function parameter is contravariant, so we cannot pass subtype(static) into supertype(short)
    // a.k1 = y; // Fails

    // x(short)  passed into b.k1(static) = contravariant, passed
    // static <: short ==> x(short) <: b.k1(static)
    // subtype(static) substituable by supertype(short)
    b.k1 = x;

    // y(static) passed into b.k1(static) = bivariant,     passed
    b.k1 = y;
}

// covariances
struct MyCoType<Mixed> {
    k1: fn() -> Mixed, // covariant over Mixed
}

fn co_example<'short>(
    mut a: MyCoType<&'short u8>,
    mut b: MyCoType<&'static u8>,
    x: fn() -> &'short u8,
    y: fn() -> &'static u8,
) {
    // a = MyCoType<short>.fn() -> short
    // b = MyCoType<static>.fn() -> static
    // x = fn() -> short
    // y = fn() -> static
    // static: short
    // short is supertype
    // static is subtype
    // static <: short

    // x() -> short passed into a.k1() -> short = bivariant, passed
    a.k1 = x;

    // y() -> static passed into a.k1() -> short = covariant, passed
    // static <: short ==> y() -> static <: a.k1() -> short
    // supertype(short) substituable by subtype(static)
    a.k1 = y;

    // x() -> short passed into b.k1() -> static = contravariant, failed
    // because function return is covariant, so we cannot pass supertype(short) into subtype(static)
    // b.k1 = x; // Fails

    // y() -> static passed into b.k1 -> static = bivariant, success
    b.k1 = y;
}

