/* It could print:
328.600498ms 158.603952ms*/
fn main() {
    use std::time::Instant;
    const SIZE: usize = 40_000;
    let start_time = Instant::now();
    let mut v = Vec::<usize>::new();
    for i in 0..SIZE {
        v.push(i);
        v.push(SIZE + i);
        v.remove(0);
        v.push(SIZE * 2 + i);
        v.remove(0);
    }
    let t1 = start_time.elapsed();
    while v.len() > 0 {
        v.remove(0);
    }
    let t2 = start_time.elapsed();
    print!("{:?} {:?}", t1, t2 - t1);
}
