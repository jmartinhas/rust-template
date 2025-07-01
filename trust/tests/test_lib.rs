/*test the src/lib.rs */

use trust::{add, divide, multiply, subtract};

#[test]
fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[test]
fn test_subtract() {
    assert_eq!(subtract(5, 3), 2);
}

#[test]
fn test_multiply() {
    assert_eq!(multiply(4, 6), 24);
}
#[test]
fn test_divide() {
    assert_eq!(divide(8, 2), Ok(4));
}
