/* It could print:
7.054µs 1.060243448s 1.02114628s*/
fn main() {
    use std::time::Instant;
    const SIZE: usize = 100_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::with_capacity(SIZE);
    let t1 = start_time.elapsed();
    for i in 0..SIZE {
        v.insert(0, i);
    }
    let t2 = start_time.elapsed();
    for _ in 0..SIZE {
        v.remove(0);
    }
    let t3 = start_time.elapsed();
    print!("{:?} {:?} {:?}", t1, t2 - t1, t3 - t2);
}
