/*create calculator using lib.rs */
use trust::{add, divide, multiply, subtract};

fn main() {
    println!("Calculator Operations:");

    let a = 10;
    let b = 5;

    println!("add({}, {}) = {}", a, b, add(a, b));
    println!("subtract({}, {}) = {}", a, b, subtract(a, b));
    println!("multiply({}, {}) = {}", a, b, multiply(a, b));

    match divide(a, b) {
        Ok(result) => println!("divide({}, {}) = {}", a, b, result),
        Err(e) => println!("divide({}, {}) error: {}", a, b, e),
    }
}
