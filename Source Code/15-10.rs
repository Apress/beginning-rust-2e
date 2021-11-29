/* It prints:
15*/
fn main() {
    fn min(arr: &[i32; 8], start: usize, count: usize) -> i32 {
        // Let's assume 'start' is between 0 and 7,
        // and 'count' is between 1 and 8 - start.
        let mut minimum = arr[start];
        for i in start + 1..start + count {
            if arr[i] < minimum { minimum = arr[i]; }
        }
        minimum
    }
    print!("{}", min(&[23, 17, 12, 16, 15, 28, 17, 30], 3, 2));
}
