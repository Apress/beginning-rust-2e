/* It prints:
1 0
2 0
3 0
... and so on, until the stack is overflowed.
Then, the following panic message will be printed:
thread 'main' has overflowed its stack
fatal runtime error: stack overflow
*/
fn main() {
    const SIZE: usize = 100_000;
    const N_ARRAY: usize = 1_000_000;
    fn create_array() -> [u8; SIZE] { [0u8; SIZE] }
    fn recursive_func(n: usize) {
        let a = create_array();
        println!("{} {}", N_ARRAY - n + 1, a[0]);
        if n > 1 { recursive_func(n - 1) }
    }
    recursive_func(N_ARRAY);
}
