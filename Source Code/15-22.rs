/* It prints:
[11, 22] [33, 44]*/
fn main() {
    let arr = [11, 22, 33, 44];
    let n = 2;
    let sr1 = &arr[0..n];
    let sr2 = &arr[n..arr.len()];
    print!("{:?} {:?}", sr1, sr2);
}
