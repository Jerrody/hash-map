fn main() {
    let mut hash_map_instance = HashMap::new();

    println!("{:?}", hash_map_instance);

    let result = hash_map_instance.get(String::from("Engi"));

    match result {
        Some(value) => println!("{}", value),
        None => println!("This table doesn't exist!"),
    }

    hash_map_instance.remove(String::from("Engi"));
    println!("{:?}", hash_map_instance);

    println!("{}", hash_map_instance.is_empty());

    hash_map_instance.insert(String::from("Engi"), String::from("Sam"));
    println!("{:?}", hash_map_instance);
    println!("{}", hash_map_instance.is_empty());
}

#[derive(Debug, Clone)]
struct HashMap<K, V>(Vec<(K, V)>);

impl<K, V> HashMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    fn new() -> Self {
        Self(vec![])
    }

    fn get(&self, key: K) -> Option<&V> {
        let table = self.0.clone();

        let index = table.into_iter().position(|(k, _)| k == key);

        index.map(|index| &self.0[index].1)
    }

    fn insert(&mut self, key: K, value: V) {
        self.0.push((key, value));
    }

    fn remove(&mut self, key: K) {
        let table = self.0.clone();

        let index = table.into_iter().position(|(k, _)| k == key);

        if let Some(index) = index {
            self.0.remove(index);
        }
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
