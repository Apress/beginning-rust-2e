/* In a 64-bit system, it prints:
8 8 8 8
In a 32-bit system, it prints:
4 4 4 4*/
fn main() {
    use std::mem::*;
    print!("{} {} {} {}",
        size_of::<isize>(),
        size_of::<usize>(),
        size_of::<&i8>(),
        size_of::<&u32>());
}
