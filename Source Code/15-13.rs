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
    let arr = [23, 17, 12, 16, 15, 2];
    let range = 2..5;
    let slice_ref = &arr[range];
    print!("{}", min(slice_ref));
}
