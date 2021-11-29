/* It may print:
0 0 0
1 1 4
5 5 8
9 9 16
17 17 32
33 33 64
65 65 128
129 129 256
257 257 512
513 513 1024
*/
fn main() {
    let mut v = vec![0; 0];
    let mut prev_capacity = std::usize::MAX;
    for i in 0..1_000 {
        let cap = v.capacity();
        if cap != prev_capacity {
            println!("{} {} {}", i, v.len(), cap);
            prev_capacity = cap;
        }
        v.push(1);
    }
}
