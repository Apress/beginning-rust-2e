/* It could print:
1.048312ms 71.587µs*/
fn main() {
    use std::time::Instant;
    const SIZE: usize = 40_000;
    let start_time = Instant::now();
    let mut vd = std::collections::VecDeque::<usize>::new();
    for i in 0..SIZE {
        vd.push_back(i);
        vd.push_back(SIZE + i);
        vd.pop_front();
        vd.push_back(SIZE * 2 + i);
        vd.pop_front();
    }
    let t1 = start_time.elapsed();
    while vd.len() > 0 {
        vd.pop_front();
    }
    let t2 = start_time.elapsed();
    print!("{:?} {:?}", t1, t2 - t1);
}
