/* It prints:
0, 1; None, Some(“Jane”)*/
fn main() {
    trait Dictionary<Key, Count> {
        fn get(&self, key: Key) -> Option<String>;
        fn count(&self, key: Key) -> Count;
    }
    struct Record {
        id: u32,
        name: String,
    }
    struct RecordSet {
        data: Vec<Record>,
    }
    impl Dictionary<u32, usize> for RecordSet {
        fn get(&self, key: u32) -> Option<String> {
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
        fn count(&self, key: u32) -> usize {
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn get_name<D>(dict: &D, id: u32) -> Option<String>
    where
        D: Dictionary<u32, usize>
    {
        dict.get(id)
    }
    let names = RecordSet {
        data: vec![
            Record { id: 34, name: "John".to_string() },
            Record { id: 49, name: "Jane".to_string() },
        ],
    };
    print!(
        "{}, {}; {:?}, {:?}",
        names.count(48),
        names.count(49),
        get_name(&names, 48),
        get_name(&names, 49));
}
