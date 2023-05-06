use rand::Error;
use std::collections::LinkedList;
use std::fs::OpenOptions;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use bincode::{deserialize, serialize_into};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
struct Item {
    key: u32,
    #[serde(with = "serde_arrays")]
    value: [char; 96],
}

impl Item {
    pub fn get_key(&self) -> &u32 {
        &self.key
    }

    pub fn get_value(&self) -> &[char; 96] {
        &self.value
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DynamicHashTable {
    table: Vec<LinkedList<Item>>,
    size: usize,
    capacity: usize,
    resize_factor: f64,
}

#[allow(dead_code)]
impl DynamicHashTable {
    pub fn new(initial_capacity: usize, resize_factor: f64) -> DynamicHashTable {
        DynamicHashTable {
            table: vec![LinkedList::new(); initial_capacity],
            size: 0,
            capacity: initial_capacity,
            resize_factor: resize_factor,
        }
    }

    pub fn save_to_file(&self, file_name: &str) -> Result<(), &'static str> {
        let file = match OpenOptions::new().write(true).create(true).open(file_name) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };

        if serialize_into(&file, &self).is_err() {
            return Err("Error serializing");
        }
        Ok(())
    }

    pub fn insert(&mut self, key: u32, value: [char; 96]) -> std::result::Result<(), &'static str> {
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

    pub fn get(&self, key: &u32) -> std::result::Result<&[char; 96], &str> {
        let index = self.hash(key) as usize;
        if let Some(list) = self.table.get(index) {
            for item in list.iter() {
                if *item.get_key() == *key {
                    return Ok(&item.value);
                }
            }
        }
        Err("Key not found")
    }

    pub fn remove(&mut self, key: &u32) -> std::result::Result<(), &str> {
        let index = self.hash(key) as usize;
        if let Some(list) = self.table.get_mut(index) {
            let mut auxiliary_list = LinkedList::new();
            while let Some(item) = list.pop_front() {
                if *item.get_key() == *key {
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

    fn hash(&self, key: &u32) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish() % self.capacity as u64
    }

    fn resize(&mut self) -> std::result::Result<(), Error> {
        self.capacity *= 2;
        let mut new_table: Vec<LinkedList<Item>> = Vec::with_capacity(self.capacity);
        new_table.extend((0..self.capacity).map(|_| LinkedList::new()));
        for item in self.table.iter().flat_map(|list| list.iter()) {
            let hash = self.hash(&item.key) as usize;
            let table_idx = hash % self.capacity;
            new_table[table_idx].push_back(Item {
                key: *item.get_key(),
                value: *item.get_value(),
            });
        }
        self.table = new_table;
        Ok(())
    }
}

pub fn read_from_file(file_name: &str) -> Option<DynamicHashTable> {
    let file = match OpenOptions::new().read(true).open(file_name) {
        Ok(file) => file,
        Err(_) => return None,
    };

    let mut buf_reader = BufReader::new(file);
    let mut buffer = Vec::new();
    if buf_reader.read_to_end(&mut buffer).is_err() {
        return None;
    }

    let table = match deserialize(&buffer) {
        Ok(table) => table,
        Err(_) => return None,
    };
    Some(table)
}