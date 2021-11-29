/* It prints:
Vec: 640: T, 917: C, 412: S, 670: T, 917: L,
    [(640, 'T'), (917, 'C'), (412, 'S'), (670, 'T'), (917, 'L')]
HashMap: 917: L, 640: T, 412: S, 670: T,
    {917: 'L', 640: 'T', 412: 'S', 670: 'T'}
BTreeMap: 412: S, 640: T, 670: T, 917: L,
    {412: 'S', 640: 'T', 670: 'T', 917: 'L'}
*/
fn main() {
    let arr = [(640, 'T'), (917, 'C'), (412, 'S'),
        (670, 'T'), (917, 'L')];
    let mut v = Vec::<_>::new();
    let mut hs = std::collections::HashMap::<_, _>::new();
    let mut bs = std::collections::BTreeMap::<_, _>::new();
    for &(key, value) in arr.iter() {
        v.push((key, value));
        hs.insert(key, value);
        bs.insert(key, value);
    }
    print!("Vec:");
    for &(key, value) in v.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", v);
    print!("HashMap:");
    for (key, value) in hs.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", hs);
    print!("BTreeMap:");
    for (key, value) in bs.iter() {
        print!(" {}: {},", key, value);
    }
    println!("\n    {:?}", bs);
}
