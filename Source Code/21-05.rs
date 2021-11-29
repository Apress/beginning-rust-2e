/* It could print:
507.821004ms 1.467µs*/
fn main() {
    use std::time::Instant;
    const SIZE: usize = 40_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE {
        v.insert(0, i);
        v.insert(0, SIZE + i);
        v.pop();
        v.insert(0, SIZE * 2 + i);
        v.pop();
    }
    let t1 = start_time.elapsed();
    while v.len() > 0 {
        v.pop();
    }
    let t2 = start_time.elapsed();
    print!("{:?} {:?}", t1, t2 - t1);
}
