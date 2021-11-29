/* It prints:
0, 1; None, Some(“Jane”)*/
fn main() {
    trait Dictionary { //1
        type Key; //2
        type Count; //3
        fn get(&self, key: Self::Key) -> Option<String>; //4
        fn count(&self, key: Self::Key) -> Self::Count; //5
    }
    struct Record {
        id: u32,
        name: String,
    }
    struct RecordSet {
        data: Vec<Record>,
    }
    impl Dictionary for RecordSet { //6
        type Key = u32; //7
        type Count = usize; //8
        fn get(&self, key: Self::Key) -> Option<String> { //9
            for record in self.data.iter() {
                if record.id == key {
                    return Some(String::from(&record.name));
                }
            }
            None
        }
        fn count(&self, key: Self::Key) -> Self::Count { //10
            let mut c = 0;
            for record in self.data.iter() {
                if record.id == key {
                    c += 1;
                }
            }
            c
        }
    }
    fn get_name<D>(
        dict: &D,
        id: <D as Dictionary>::Key, //11
    ) -> Option<String>
    where
        D: Dictionary, //12
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
