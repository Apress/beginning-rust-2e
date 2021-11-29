/* It prints:
None, Some(“Jane”)*/
fn main() {
    trait Dictionary<Key> {
        fn get(&self, key: Key) -> Option<String>;
    }
    struct Record {
        id: u32,
        name: String,
    }
    struct RecordSet {
        data: Vec<Record>,
    }
    impl Dictionary<u32> for RecordSet {
        fn get(&self, key: u32) -> Option<String> {
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
    }
    fn get_name<D>(dict: &D, id: u32) -> Option<String>
    where
        D: Dictionary<u32>,
    {
        dict.get(id)
    }
    let names = RecordSet {
        data: vec![
            Record { id: 34, name: "John".to_string() },
            Record { id: 49, name: "Jane".to_string() },
        ],
    };
    print!("{:?}, {:?}",
        get_name(&names, 48), get_name(&names, 49));
}
