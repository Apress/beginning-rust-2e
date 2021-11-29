/* It prints:
0: 97
1: 98
2: 99
3: 48
4: 49
5: 50
6: 195
7: 168
8: 226
9: 130
10: 172
*/
fn main() {
    let s = "abc012è€";
    for i in 0..s.len() {
        println!("{}: {}", i, s.as_bytes()[i]);
    }
}
