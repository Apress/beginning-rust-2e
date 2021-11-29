/* It prints:
48 35 20 39 42 33 50 29 27 18 17 16 13 12 */
fn main() {
    let data = vec![
        48, 18, 20, 35, 17, 13, 39, 12, 42, 33, 29, 27, 50, 16];
    use std::collections::BinaryHeap;
    let mut v = BinaryHeap::<i32>::new();
    fn add(v: &mut BinaryHeap<i32>, item: i32) {
        v.push(item);
    }
    fn extract(v: &mut BinaryHeap<i32>) -> i32 {
        v.pop().unwrap()
    }
    let mut i = 0;
    loop {
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        if i == data.len() { break; }
        add(&mut v, data[i]);
        i += 1;
        print!("{} ", extract(&mut v));
    }
    while ! v.is_empty() {
        print!("{} ", extract(&mut v));
    }
}
