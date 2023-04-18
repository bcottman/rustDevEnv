
extern crate num_cpus;

fn main() {
    println!("Hello, world!");
}
//
fn num_cores() -> usize {
    num_cpus::get()
}

//

//
fn fibonacci(n: i32) -> i32 {
    if n < 0 {
        panic!("Fibonacci sequence is only defined for non-negative numbers");
    }
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
#[test]
fn test_fbonacci_first() {
    assert_eq!(fibonacci(0), 0);
}
#[test]
fn test_fbonacci_fail() {
    assert_eq!(fibonacci(0), -1);
}
#[test]
fn test_fbonacci_rest() {
    assert_eq!(fibonacci(1), 1);
    assert_eq!(fibonacci(2), 1);
    assert_eq!(fibonacci(3), 2);
    assert_eq!(fibonacci(4), 3);
    assert_eq!(fibonacci(5), 5);
    assert_eq!(fibonacci(6), 8);
    assert_eq!(fibonacci(7), 13);
}

