/* It prints:
[45, 12, 10, 8, 7, 4, 1, 0]*/
fn main() {
    let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
    use std::cmp::Ordering;
    let desc = |a: &i32, b: &i32| -> Ordering {
        if a < b { Ordering::Greater }
        else if a > b { Ordering::Less }
        else { Ordering::Equal }
    };
    arr.sort_by(desc);
    print!("{:?}", arr);
}
