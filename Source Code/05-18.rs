/* It prints:
15, 8, 4.*/
fn main() {
    let x = [[[0; 4]; 8]; 15];
    print!("{}, {}, {}.",
        x.len(), x[0].len(), x[0][0].len());
}
