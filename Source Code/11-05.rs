/* It prints:
1 0
2 0
3 0
... and so on, until the memory of the process is overflowed.
Then, a runtime error message will be printed.
*/
fn main() {
    const SIZE: usize = 100_000;
    const N_ARRAY: usize = 1_000_000;
    fn create_array() -> Box<[u8; SIZE]> { Box::new([0u8; SIZE]) }
    fn recursive_func(n: usize) {
        let a = create_array();
        println!("{} {}", N_ARRAY - n + 1, a[0]);
        if n > 1 { recursive_func(n - 1) }
    }
    recursive_func(N_ARRAY);
}
