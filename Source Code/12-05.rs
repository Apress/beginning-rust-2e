/* It prints:
1 1 2 2 4 4 8 8 16 16 4 8 1 4*/
fn main() {
    use std::mem::*;
    print!("{} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        size_of::<i8>(),
        size_of::<u8>(),
        size_of::<i16>(),
        size_of::<u16>(),
        size_of::<i32>(),
        size_of::<u32>(),
        size_of::<i64>(),
        size_of::<u64>(),
        size_of::<i128>(),
        size_of::<u128>(),
        size_of::<f32>(),
        size_of::<f64>(),
        size_of::<bool>(),
        size_of::<char>());
}
