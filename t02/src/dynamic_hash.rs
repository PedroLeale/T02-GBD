use std::fs::{OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};


const ITEMS_PER_PAGE: usize = 1;
const EMPTY_ITEM_KEY: u32 = 0xffffffff;
const EMPTY_ITEM_VALUE: [char; 96] = ['x'; 96];

#[derive(Clone, Copy, Debug)]
pub struct Item {
    key: u32,
    value: [char; 96],
}

#[derive(Debug)]
pub struct DynamicHashTable {
    size: usize,
    capacity: usize,
    file_name: String,
}
//Vec<[Item; ITEMS_PER_PAGE]>
#[allow(dead_code)]
impl DynamicHashTable {
    pub fn new(
        initial_capacity: usize,
        file_name: String,
    ) -> Result<DynamicHashTable, &'static str> {
        let _table = match OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_name.clone())
        {
            //Save a empty hash table to the file
            //The size is the "initial_capacity"
            Ok(mut file) => {
                let mut buffer: Vec<u8> = Vec::new();
                for _i in 0..initial_capacity {
                    for _j in 0..ITEMS_PER_PAGE {
                        let mut empty_key = EMPTY_ITEM_KEY.to_be_bytes().to_vec();
                        let mut empty_value =
                            EMPTY_ITEM_VALUE.iter().collect::<String>().into_bytes();
                        buffer.append(&mut empty_key); //Rust doesn't support NULL so
                        buffer.append(&mut empty_value); //This is my workaround
                    }
                }
                match file.write_all(&buffer) {
                    Ok(_) => {
                        return Ok(DynamicHashTable {
                            size: 0,
                            capacity: initial_capacity,
                            file_name,
                        })
                    }
                    Err(_) => {
                        return Err("Error writing empty table with initial capacity to file")
                    }
                }
            }
            Err(_) => return Err("Error creating hash table"),
        };
    }

    pub fn print_all_table(&self) {
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name.clone())
        {
            Ok(file) => file,
            Err(_) => return,
        };

        let mut buffer = [0u8; 100 * ITEMS_PER_PAGE];
        let mut offset = 0;
        loop {
            match file.seek(SeekFrom::Start(offset)) {
                Ok(_) => {
                    match file.read_exact(&mut buffer) {
                        Ok(_) => {
                            // Read the whole page and find an empty slot
                            // In the first empty slot, we insert the key + value pair
                            for chunk in buffer.chunks_exact(100) {
                                let (key_buf, value_buf) =
                                    chunk.split_at(std::mem::size_of::<u32>());
                                let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                                let registro_value = value_buf
                                    .iter()
                                    .map(|&c| c as char)
                                    .collect::<String>()
                                    .into_bytes();
                                println!(
                                    "Key: {}, Value: {}",
                                    registro_key,
                                    String::from_utf8(registro_value).unwrap()
                                );
                            }
                        }
                        Err(_) => return,
                    }
                }
                Err(_) => return,
            }
            offset += 100 * ITEMS_PER_PAGE as u64;
        }
    }

    pub fn insert(&mut self, key: u32, value: [char; 96]) -> Result<(), &'static str> {
        let mut file = match OpenOptions::new()
            .read(true)
            .write(true)
            .open(self.file_name.clone())
        {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };
        let mut buffer = [0u8; 100 * ITEMS_PER_PAGE];
        let mut offset = self.hash(key) * 100 * ITEMS_PER_PAGE as u64;
        match file.seek(SeekFrom::Start(offset)) {
            Ok(_) => {
                match file.read(&mut buffer) {
                    Ok(_) => {
                        //Buffer now holds the whole page
                        //Now we need to split it into 100 byte chunks
                        for buffer in buffer.chunks(100) {
                            let (key_buf, _) = buffer.split_at(std::mem::size_of::<u32>());
                            let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                            if registro_key == EMPTY_ITEM_KEY {
                                let mut insert_buffer = key.to_be_bytes().to_vec();
                                insert_buffer
                                    .append(&mut value.iter().collect::<String>().into_bytes());
                                file.seek(SeekFrom::Start(offset)).unwrap();
                                match file.write_all(&insert_buffer) {
                                    Ok(_) => {
                                        return Ok(());
                                    }
                                    Err(_) => {
                                        return Err("Error writing key value pair to file");
                                    }
                                }
                            }
                            offset += 100;
                        }
                        match self.resize_and_insert(key, value) {
                            Ok(_) => return Ok(()),
                            Err(_) => return Err("Error resizing and inserting"),
                        }
                    }
                    Err(_) => return Err("Error reading file"),
                }
            }
            Err(_) => return Err("Error seeking file"),
        }
    }

    pub fn read_key(&self, key: u32) -> Result<Item, &'static str> {
        let mut file = match OpenOptions::new().read(true).open(self.file_name.clone()) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };
        let mut buffer = [0u8; 100 * ITEMS_PER_PAGE];
        let offset = self.hash(key) * 100 * ITEMS_PER_PAGE as u64;
        match file.seek(SeekFrom::Start(offset)) {
            Ok(_) => {
                match file.read_exact(&mut buffer) {
                    Ok(_) => {
                        //Buffer now holds the whole page
                        //Now we need to split it into 100 byte chunks (Register size)
                        for chunk in buffer.chunks_exact(100) {
                            let (key_buf, value_buf) = chunk.split_at(std::mem::size_of::<u32>());
                            let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                            if registro_key == key {
                                let value_buf = String::from_utf8(
                                    value_buf
                                        .iter()
                                        .map(|&c| c as char)
                                        .collect::<String>()
                                        .into_bytes(),
                                )
                                .unwrap();
                                let item = Item {
                                    key: registro_key,
                                    value: value_buf
                                        .chars()
                                        .take(96)
                                        .collect::<Vec<char>>()
                                        .try_into()
                                        .unwrap(),
                                };
                                file.rewind().unwrap();
                                return Ok(item);
                            }
                        }
                    }
                    Err(_) => return Err("Error reading file"),
                }
            }
            Err(_) => return Err("Error seeking file"),
        }
        file.rewind().unwrap();
        Err("Error reading key")
    }

    pub fn remove_key(&mut self, key: u32) -> Result<(), &'static str> {
        let mut file = match OpenOptions::new().read(true).write(true).open(self.file_name.clone()) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };
        let mut buffer = [0u8; 100 * ITEMS_PER_PAGE];
        let mut offset = self.hash(key) * 100 * ITEMS_PER_PAGE as u64;
        match file.seek(SeekFrom::Start(offset)) {
            Ok(_) => {
                match file.read(&mut buffer) {
                    Ok(_) => {
                        //Buffer now holds the whole page
                        //Now we need to split it into 100 byte chunks
                        for buffer in buffer.chunks(100) {
                            let (key_buf, _) = buffer.split_at(std::mem::size_of::<u32>());
                            let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                            if registro_key == key {
                                println!("Removing key: {}", key);
                                let mut insert_buffer = EMPTY_ITEM_KEY.to_be_bytes().to_vec();
                                insert_buffer
                                    .append(&mut EMPTY_ITEM_VALUE.iter().collect::<String>().into_bytes());
                                file.seek(SeekFrom::Start(offset)).unwrap();
                                match file.write_all(&insert_buffer) {
                                    Ok(_) => {
                                        return Ok(());
                                    }
                                    Err(_) => {
                                        return Err("Error removing key value pair");
                                    }
                                }
                            }
                            offset += 100;
                        }
                        Ok(())
                    }
                    Err(_) => return Err("Error reading file"),
                }
            }
            Err(_) => return Err("Error seeking file"),
        }
    }

    fn resize_and_insert(&mut self, key: u32, value: [char; 96]) -> Result<(), &'static str> {
        let old_table = self.read_all_table();
        self.capacity = self.capacity * 2;
        //now new_table with double the size of the old
        let mut new_table = vec![
            [Item {
                key: EMPTY_ITEM_KEY,
                value: EMPTY_ITEM_VALUE,
            }; ITEMS_PER_PAGE * 2];
            self.capacity
        ];
        //now we need to rehash all the items from the old table to the new one
        for i in 0..old_table.len() {
            for j in 0..ITEMS_PER_PAGE {
                if old_table[i][j].key != EMPTY_ITEM_KEY {
                    let new_index = self.hash(old_table[i][j].key);
                    new_table[new_index as usize][j] = old_table[i][j];
                } else {
                    continue;
                }
            }
        }
        //now we need to insert the new item
        let new_index = self.hash(key);
        for i in 0..ITEMS_PER_PAGE {
            if new_table[new_index as usize][i].key == EMPTY_ITEM_KEY {
                new_table[new_index as usize][i] = Item { key, value };
                break;
            }
        }
        //Now we just need to overwrite the old file
        let mut file = match OpenOptions::new()
            .write(true)
            .open(self.file_name.clone())
        {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };
        file.rewind().unwrap();
        let mut buffer = Vec::new();
        for i in 0..new_table.len() {
            for j in 0..ITEMS_PER_PAGE {
                buffer.append(&mut new_table[i][j].key.to_be_bytes().to_vec());
                buffer.append(
                    &mut new_table[i][j]
                        .value
                        .iter()
                        .collect::<String>()
                        .into_bytes(),
                );
            }
        }
        match file.write_all(&buffer) {
            Ok(_) => {
                return Ok(());
            }
            Err(_) => return Err("Error writing key value pair to file"),
        }
    }

    pub fn read_all_table(&mut self) -> Vec<[Item; ITEMS_PER_PAGE]> {
        let mut new_table: Vec<[Item; ITEMS_PER_PAGE]> = Vec::new();
        let mut new_table_item: [Item; ITEMS_PER_PAGE] = [Item {
            key: EMPTY_ITEM_KEY,
            value: EMPTY_ITEM_VALUE,
        }; ITEMS_PER_PAGE];
        let mut file = match OpenOptions::new().read(true).open(self.file_name.clone()) {
            Ok(file) => file,
            Err(_) => return new_table,
        };

        let mut buffer = [0u8; 100 * ITEMS_PER_PAGE];
        let mut offset = 0;

        loop {
            match file.seek(SeekFrom::Start(offset)) {
                Ok(_) => {
                    match file.read_exact(&mut buffer) {
                        Ok(_) => {
                            // Read the whole page and find an empty slot
                            // In the first empty slot, we insert the key + value pair
                            let mut i = 0;
                            for chunk in buffer.chunks_exact(100) {
                                let (key_buf, value_buf) =
                                    chunk.split_at(std::mem::size_of::<u32>());
                                let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                                let registro_value = value_buf
                                    .iter()
                                    .map(|&c| c as char)
                                    .collect::<String>()
                                    .into_bytes();
                                new_table_item[i] = Item {
                                    key: registro_key,
                                    value: registro_value
                                        .iter()
                                        .map(|&c| c as char)
                                        .collect::<String>()
                                        .chars()
                                        .take(96)
                                        .collect::<Vec<char>>()
                                        .try_into()
                                        .unwrap(),
                                };
                                i += 1;
                            }
                        }
                        Err(_) => return new_table,
                    }
                }
                Err(_) => return new_table,
            }
            new_table.push(new_table_item);
            offset += 100 * ITEMS_PER_PAGE as u64;
        }
    }

    /*pub fn insert(&mut self, key: u32, value: [char; 96]) -> std::result::Result<(), &'static str> {
        let index = self.hash(key) as usize;
        let item = Item { key, value };

        if let Some(list) = self.table.get_mut(index) {
            for i in 0..ITEMS_PER_PAGE {
                if list[i].is_empty() {
                    list[i] = item;
                    self.size += 1;
                    return Ok(());
                }
            }
            self.table.push(
                [Item {
                    key: 0xFFFFFFFF,
                    value: ['a'; 96],
                }; ITEMS_PER_PAGE],
            );
            self.table.last_mut().unwrap()[0] = item;
            self.size += 1;
            return Ok(());
        }
        Err("Error inserting")
    }*/

    /*pub fn remove(&mut self, key: u32) -> std::result::Result<(), &'static str> {
        let index = self.hash(key) as usize;
        if let Some(list) = self.table.get_mut(index) {
            for i in 0..ITEMS_PER_PAGE {
                if !list[i].is_empty() {
                    let item = list[i];
                    if item.key == key {
                        list[i] = Item {
                            key: 0xFFFFFFFF,
                            value: ['a'; 96],
                        };
                        self.size -= 1;
                        return Ok(());
                    }
                }
            }
        }
        Err("Key not found")
    }*/

    pub fn get_size(&self) -> usize {
        self.size
    }

    fn hash(&self, key: u32) -> u64 {
        (key % self.capacity as u32) as u64
    }
}

/*
    pub fn read_key(&self, key: u32) -> Result<u64, &'static str> {
        let mut file = match OpenOptions::new().read(true).open(self.file_name.clone()) {
            Ok(file) => file,
            Err(_) => return Err("Error opening file"),
        };

        let mut buffer = [0u8; 12 * ITEMS_PER_PAGE];
        let offset = self.hash(key) * 12 * ITEMS_PER_PAGE as u64;

        match file.seek(SeekFrom::Start(offset)) {
            Ok(_) => {
                match file.read(&mut buffer) {
                    Ok(_) => {
                        //Buffer now holds the whole page
                        //Now we need to split it into 12 byte chunks
                        for buffer in buffer.chunks_exact(12) {
                            let (key_buf, value_buf) = buffer.split_at(std::mem::size_of::<u32>());
                            let registro_key = u32::from_be_bytes(key_buf.try_into().unwrap());
                            if registro_key == key {
                                let registro_value =
                                    u64::from_be_bytes(value_buf.try_into().unwrap());
                                return Ok(registro_value);
                            }
                        }
                    }
                    Err(_) => return Err("Error reading file"),
                }
            }
            Err(_) => return Err("Error seeking file"),
        }

        Err("Error reading key")
    }
*/
