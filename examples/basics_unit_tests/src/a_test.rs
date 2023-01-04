
use crate::a;

#[test]
fn test_add() {
    assert!(5 == a::add(3, 2));
    assert_eq!(5, a::add(3, 2));
    assert_ne!(2, a::add(3, 2));

    if a::add(3, 2) != 5 {
        panic!("unexpected");
    }

}