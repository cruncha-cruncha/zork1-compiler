use std::{
    fmt,
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

pub type FileKey = usize;

pub struct FileTable {
    index: usize,
    table: Vec<FileInfo>,
}

#[derive(Clone, PartialEq)]
pub struct FileInfo {
    pub name: String,
    pub path: Box<Path>,
}

impl FileTable {
    pub fn new() -> FileTable {
        FileTable {
            index: 0,
            table: Vec::new(),
        }
    }

    pub fn add(&mut self, path: PathBuf) -> FileKey {
        self.table.push(FileInfo {
            name: path.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.into_boxed_path(),
        });

        self.table.len() - 1
    }

    pub fn get(&self, k: FileKey) -> Option<FileInfo> {
        if self.table.len() <= k {
            return None;
        }

        Some(self.table[k].clone())
    }

    #[allow(dead_code)]
    pub fn contains(&self, name: &str) -> Option<FileKey> {
        for (i, info) in self.table.iter().enumerate() {
            if info.name == name {
                return Some(i);
            }
        }

        None
    }

    #[allow(dead_code)]
    pub fn format_location(&self, location: &impl FileTableLocation) -> String {
        let info = match self.get(location.get_file_key()) {
            Some(v) => v,
            None => return String::from("UNKNOWN_FILE"),
        };

        format!(
            "in {} at line {}, char {}",
            info.name,
            location.get_line_number(),
            location.get_char_number()
        )
    }
}

pub trait FileTableLocation {
    fn get_file_key(&self) -> FileKey;
    fn get_line_number(&self) -> u64;
    fn get_char_number(&self) -> u64;
}

pub fn format_file_location(location: &impl FileTableLocation) -> String {
    format!(
        "in file {} at line {}, char {}",
        location.get_file_key(),
        location.get_line_number(),
        location.get_char_number()
    )
}

impl Iterator for FileTable {
    type Item = (FileKey, Box<Path>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.table.len() {
            return None;
        }

        let index = self.index;
        let item = &self.table[index];
        self.index += 1;

        Some((index, item.path.clone()))
    }
}

impl fmt::Display for FileTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        for (i, value) in self.table.iter().enumerate() {
            out.push_str(&format!("{}: {}\n", i, &value.name));
        }
        write!(f, "FileTable\n{}", out)
    }
}

#[allow(non_snake_case)]
pub fn get_BufReader(file_path: &Path) -> Option<BufReader<File>> {
    match File::open(file_path) {
        Ok(f) => Some(BufReader::new(f)),
        Err(e) => {
            println!("Failed to open file {}", file_path.to_str().unwrap());
            println!("{}", e);
            None
        }
    }
}
