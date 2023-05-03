use rand::Error;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::fs::{read, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Write, Read};
extern crate savefile;
use bincode::{deserialize, deserialize_from, serialize_into};
use serde::{Deserialize, Serialize, Serializer};
use serde_cbor::de::from_reader;

const INITIAL_CAPACITY: usize = 100;
const LOAD_FACTOR: f64 = 0.75;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Item<K, V> {
    key: K,
    value: V,
}

#[allow(dead_code)]
impl<K, V> Item<K, V>
where
    K: Clone,
    V: Clone,
{
    pub fn get_key(&self) -> &K {
        &self.key
    }

    pub fn get_value(&self) -> &V {
        &self.value
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicHashTable<K, V> {
    table: Vec<LinkedList<Item<K, V>>>,
    size: usize,
    capacity: usize,
    resize_factor: f64,
}

#[allow(dead_code)]
impl<'a, K, V> DynamicHashTable <K, V>
where
    K: Hash + Eq + Debug + Clone,
    V: Debug + Clone,
{
    pub fn new() -> DynamicHashTable<K, V> {
        DynamicHashTable {
            table: vec![LinkedList::new(); INITIAL_CAPACITY],
            size: 0,
            capacity: INITIAL_CAPACITY,
            resize_factor: LOAD_FACTOR,
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> Result<(), &'static str>
    where
        K: Serialize,
        V: Serialize,
    {
        let file = match OpenOptions::new().write(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };

        if serialize_into(&file, &self).is_err() {
            return Err("Error serializing");
        }
        Ok(())
    }

    /*pub fn read_from_file (
        &mut self,
        file_name: &str,
    ) -> Result<DynamicHashTable<K, V>, &'static str>
    where
        K: Deserialize<'static>,
        V: Deserialize<'static>,
    {
        let file = match OpenOptions::new().read(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };

        let mut buf_reader = BufReader::new(file);
        let mut buffer = Vec::new();
        if let Err(_) = buf_reader.read_to_end(&mut buffer) {
            return Err("Error reading from file");
        }

        let table = match deserialize(&buffer) {
            Ok(table) => table,
            Err(_) => return Err("Error deserializing"),
        };
        Ok(table)
    }*/

    pub fn insert(&mut self, key: K, value: V) -> std::result::Result<(), &'static str> {
        if self.size as f64 / self.capacity as f64 > self.resize_factor && self.resize().is_err() {
            return Err("Error resizing table");
        }

        let index = self.hash(&key) as usize;
        let item = Item { key, value };

        match self.table.get_mut(index) {
            Some(list) => {
                list.push_back(item);
            }
            None => {
                let mut list = LinkedList::new();
                list.push_back(item);
                self.table.insert(index, list);
            }
        }
        self.size += 1;
        Ok(())
    }

    pub fn get(&self, key: &K) -> std::result::Result<&V, &str> {
        let index = self.hash(key) as usize;
        if let Some(list) = self.table.get(index) {
            for item in list.iter() {
                if item.get_key().clone() == *key {
                    return Ok(&item.value);
                }
            }
        }
        Err("Key not found")
    }

    pub fn remove(&mut self, key: &K) -> std::result::Result<(), &str> {
        let index = self.hash(key) as usize;
        if let Some(list) = self.table.get_mut(index) {
            let mut auxiliary_list = LinkedList::new();
            while let Some(item) = list.pop_front() {
                if item.get_key().clone() == *key {
                    self.size -= 1;
                    list.append(&mut auxiliary_list);
                    return Ok(());
                } else {
                    auxiliary_list.push_back(item);
                }
            }
            list.append(&mut auxiliary_list);
        }
        Err("Key not found")
    }

    fn hash(&self, key: &K) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % self.capacity as u64
    }

    fn resize(&mut self) -> std::result::Result<(), Error> {
        self.capacity *= 2;
        let mut new_table: Vec<LinkedList<Item<K, V>>> = Vec::with_capacity(self.capacity);
        new_table.extend((0..self.capacity).map(|_| LinkedList::new()));
        for item in self.table.iter().flat_map(|list| list.iter()) {
            let hash = self.hash(&item.key) as usize;
            let table_idx = hash % self.capacity;
            new_table[table_idx].push_back(Item {
                key: item.get_key().clone(),
                value: item.get_value().clone(),
            });
        }
        self.table = new_table;
        Ok(())
    }
}
