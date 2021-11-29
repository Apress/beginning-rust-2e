/* It could prints:
1599960000 593.578µs 556.144µs 18.647µs 81.504µs*/
fn main() {
    use std::time::Instant;
    const SIZE: usize = 40_000;
    let mut v = Vec::<usize>::new();
    let mut vd = std::collections::VecDeque::<usize>::new();
    let start_time = Instant::now();
    for i in 0..SIZE {
        v.push(i);
    }
    let t1 = start_time.elapsed();
    for i in 0..SIZE {
        vd.push_back(i);
    }
    let mut count = 0;
    let t2 = start_time.elapsed();
    for i in v.iter() {
        count += i;
    }
    let t3 = start_time.elapsed();
    for i in vd.iter() {
        count += i;
    }
    let t4 = start_time.elapsed();
    print!("{} {:?} {:?} {:?} {:?}", count,
        t1, t2 - t1, t3 - t2, t4 - t3);
}
