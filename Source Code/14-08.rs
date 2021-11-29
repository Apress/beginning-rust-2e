/* In a 64-bit system, it may print:
0x1 0 0
0x55f7d528f9d0 8 1
0x55f7d528f9d0 8 2
0x55f7d528f9d0 8 3
0x55f7d528f9d0 8 4
0x55f7d528f9d0 8 5
0x55f7d528f9d0 8 6
0x55f7d528f9d0 8 7
0x55f7d528f9d0 8 8
0x55f7d528f9d0 16 9
0x55f7d528f9d0 16 10
0x55f7d528f9d0 16 11
0x55f7d528f9d0 16 12
0x55f7d528f9d0 16 13
0x55f7d528f9d0 16 14
0x55f7d528f9d0 16 15
0x55f7d528f9f0
0x55f7d528fa10 32 17: aaaaaaaaaaaaaaaa-
*/
fn main() {
    let mut s1 = "".to_string();
    for _ in 0..16 {
        println!("{:p} {} {}",
            s1.as_ptr(), s1.capacity(), s1.len());
        s1.push('a');
    }
    let s2 = "x".to_string();
    s1.push('-');
    println!("{:p}", s2.as_ptr());
    println!("{:p} {} {}: {}",
        s1.as_ptr(), s1.capacity(), s1.len(), s1);
}
