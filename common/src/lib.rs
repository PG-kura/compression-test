use anyhow::Result;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub trait Sized {
    fn len(&self) -> usize;
}

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Origin {
    pub files: HashMap<String, Vec<u8>>,
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

impl Sized for Origin {
    fn len(&self) -> usize {
        self.amount_size
    }
}

pub type Header = Vec<(String, usize)>;

pub struct Compressed {
    pub buffer: Vec<u8>,
    pub header: Header,
}

impl Sized for Compressed {
    fn len(&self) -> usize {
        self.buffer.len()
    }
}

#[derive(Clone)]
pub struct Archive {
    pub path: PathBuf,
    pub size: usize,
}

impl Archive {
    pub fn new<P>(path: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let size = fs::metadata(&path)?.len();
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            size: size as usize,
        })
    }
}

impl Sized for Archive {
    fn len(&self) -> usize {
        self.size
    }
}
