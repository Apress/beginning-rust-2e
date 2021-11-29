/* It could print:
Pushes into Vec: 11ns per item
Insertions into HashSet: 152ns per item
Insertions into BTreeSet: 145ns per item
Linear search in Vec: 9.041µs per item
Sorting of Vec: 131.302µs
Binary search in Vec: 35ns per item
Search in HashSet: 24ns per item
Search in BTreeSet: 41ns per item
*/
fn main() {
    const SIZE: u32 = 40_000;
    // Creating empty collections.
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashSet::<_>::new();
    let mut bs = std::collections::BTreeSet::<_>::new();
    // Adding elements to the collections,
    // measuring the elapsed time.
    let start_time = std::time::Instant::now();
    for i in 0..SIZE { v.push(i); }
    let t1 = start_time.elapsed();
    for i in 0..SIZE { hs.insert(i); }
    let t2 = start_time.elapsed();
    for i in 0..SIZE { bs.insert(i); }
    let t3 = start_time.elapsed();
    for i in 0..SIZE { if ! v.contains(&i) { return; } }
    let t4 = start_time.elapsed();
    // Performing binary searches on a sorted vector,
    // measuring the elapsed time.
    v.swap(10_000, 20_000);
    v.sort();
    let t5 = start_time.elapsed();
    for i in 0..SIZE {
        if v.binary_search(&i).is_err() { return; }
    }
    // Searching the hashtable, measuring the elapsed time.
    let t6 = start_time.elapsed();
    for i in 0..SIZE { if ! hs.contains(&i) { return; } }
    // Searching the BTree, measuring the elapsed time.
    let t7 = start_time.elapsed();
    for i in 0..SIZE { if ! bs.contains(&i) { return; } }
    let t8 = start_time.elapsed();
    // Printing the measured times.
    println!("Pushes into Vec: {:?} per item", t1 / SIZE);
    println!("Insertions into HashSet: {:?} per item", (t2 - t1) / SIZE);
    println!("Insertions into BTreeSet: {:?} per item", (t3 - t2) / SIZE);
    println!("Linear search in Vec: {:?} per item", (t4 - t3) / SIZE);
    println!("Sorting of Vec: {:?}", t5 - t4);
    println!("Binary search in Vec: {:?} per item", (t6 - t5) / SIZE);
    println!("Search in HashSet: {:?} per item", (t7 - t6) / SIZE);
    println!("Search in BTreeSet: {:?} per item", (t8 - t7) / SIZE);
}
