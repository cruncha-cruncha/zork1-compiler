use std::collections::HashMap;

pub struct FileTable {
    key: u32,
    table: HashMap<u32, String>,
}

impl FileTable {
    pub fn new() -> FileTable {
        FileTable {key: 0, table: HashMap::new()}
    }

    pub fn insert(&mut self, v: String) -> u32 {
        self.key += 1;
        self.table.insert(
            self.key,
            v,
        );
        self.key
    }

    pub fn get(&mut self, k: u32) -> Option<String> {
        match self.table.get(&k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn find_key(&mut self, v: String) -> Option<u32> {
        for (key, value) in self.table.iter() {
            if *value == v {
                return Some(*key);
            }
        }

        None
    }
}
