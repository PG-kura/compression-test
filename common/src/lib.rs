use std::collections::HashMap;
use std::fmt;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Origin {
    files: HashMap<String, Vec<u8>>,
    amount_size: usize,
}

impl Origin {
    pub fn insert(&mut self, name: String, value: Vec<u8>) {
        self.amount_size += value.len();
        self.files.insert(name, value);
    }

    pub fn amount_size(&self) -> usize {
        self.amount_size
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Vec<u8>)> {
        self.files.iter()
    }

    pub fn get(&self, name: &str) -> Option<&Vec<u8>> {
        self.files.get(name)
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_s: String = humansize::format_size(self.amount_size, humansize::DECIMAL);
        write!(f, "{size_s}")
    }
}

pub type Header = Vec<(String, usize)>;

pub struct Archive {
    pub buffer: Vec<u8>,
    pub header: Header,
}

impl Archive {
    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl fmt::Display for Archive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let size_s: String = humansize::format_size(self.len(), humansize::DECIMAL);
        write!(f, "{size_s}")
    }
}
