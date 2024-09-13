use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

pub struct Formatter {
    file: BufWriter<File>,
    tab_count: u64,
    spacer: String,
}

impl Formatter {
    pub fn new(path: &Path) -> Formatter {
        let file = match File::create(path) {
            Ok(f) => BufWriter::new(f),
            Err(_) => panic!("Could not create file"),
        };

        Formatter {
            file,
            tab_count: 0,
            spacer: String::new(),
        }
    }

    pub fn indent(&mut self) {
        self.tab_count += 1;
        self.refresh_spacer();
    }

    pub fn outdent(&mut self) {
        self.tab_count -= 1;
        self.refresh_spacer();
    }

    fn refresh_spacer(&mut self) {
        self.spacer = String::new();
        for _ in 0..self.tab_count {
            self.spacer.push_str("  ");
        }
    }

    pub fn write(&mut self, s: &str, with_tabs: bool) -> Result<(), std::io::Error> {
        if with_tabs {
            self.file.write_all(self.spacer.as_bytes())?;
        }

        self.file.write_all(s.as_bytes())
    }

    pub fn newline(&mut self) -> Result<(), std::io::Error> {
        self.write("\n", false)
    }

    pub fn writeln(&mut self, s: &str) -> Result<(), std::io::Error> {
        self.write(s, true)?;
        self.write("\n", false)
    }

    pub fn flush(&mut self) -> Result<(), std::io::Error> {
        self.file.flush()
    }

    pub fn safe_case(input: &str) -> String {
        let mut result = String::new();
        let mut capitalize = false;

        if input.chars().next().unwrap_or_default().is_numeric() {
            result.push('n');
        }

        for c in input.chars() {
            if c == '?' {
                result.push('Q');
            } else if c == '-' {
                capitalize = true;
            } else if !c.is_alphanumeric() {
                result.push('_');
            } else if capitalize {
                result.push(c.to_ascii_uppercase());
                capitalize = false;
            } else {
                result.push(c.to_ascii_lowercase());
            }
        }

        result
    }

    #[allow(dead_code)]
    pub fn safe_case_option(input_option: &Option<String>) -> String {
        match input_option {
            Some(input) => Formatter::safe_case(input),
            None => String::new(),
        }
    }
}
