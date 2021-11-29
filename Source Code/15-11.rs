/* It prints:
12*/
fn main() {
    fn min(arr: &[i32]) -> i32 {
        // Let's assume 'arr' is not empty.
        let mut minimum = arr[0];
        for i in 1..arr.len() {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30]));
}
