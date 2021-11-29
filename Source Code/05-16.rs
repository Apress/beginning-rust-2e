/* It prints:
1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, */
fn main() {
    let mut fib = [1; 12];
    for i in 2..fib.len() {
        fib[i] = fib[i - 2] + fib[i - 1];
    }
    for i in 0..fib.len() {
        print!("{}, ", fib[i]);
    }
}
