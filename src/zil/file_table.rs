use std::collections::HashMap;
use std::fmt;

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

    #[allow(dead_code)]
    pub fn get(&mut self, k: u32) -> Option<String> {
        match self.table.get(&k) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    #[allow(dead_code)]
    pub fn find_key(&mut self, v: String) -> Option<u32> {
        for (key, value) in self.table.iter() {
            if *value == v {
                return Some(*key);
            }
        }

        None
    }
}

impl fmt::Display for FileTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (key, value) in self.table.iter() {
            out.push_str(&format!("  {0:>4}: {1}\n", &key, &value));
        }
        write!(f, "FileTable\n{}", out)
    }
}
