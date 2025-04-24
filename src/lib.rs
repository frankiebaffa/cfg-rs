#[cfg(test)]
mod test;

use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::{ BufReader, BufRead, Read, self, },
    path::PathBuf,
};

#[derive(Default)]
pub struct Config {
    kvs: HashMap<String, Vec<String>>,
}

impl Config {
    pub fn add_value<K: AsRef<str>, V: AsRef<str>>(&mut self, k: K, v: V) {
        let key = k.as_ref();
        let value = v.as_ref();

        if !self.kvs.contains_key(key) {
            self.kvs.insert(key.to_owned(), Vec::new());
        }

        self.kvs.get_mut(key).unwrap().push(value.to_owned());
    }

    pub fn parse_buffered<R: Read>(contents: BufReader<R>) -> io::Result<Self> {
        let mut cfg = Config::default();

        for line_res in contents.lines() {
            let line = line_res?;

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let mut key_val_split = line.split("=");

            let key = key_val_split.next().unwrap_or("");
            let value = key_val_split.collect::<String>();

            cfg.add_value(key, value);
        }

        Ok(cfg)
    }

    pub fn parse<R: Read>(contents: R) -> io::Result<Self> {
        Self::parse_buffered(BufReader::new(contents))
    }

    pub fn from_string<S: AsRef<str>>(contents: S) -> io::Result<Self> {
        let bytes = contents.as_ref().as_bytes();
        Self::parse(bytes)
    }

    pub fn from_file<P: Into<PathBuf>>(path: P) -> io::Result<Self> {
        let path: PathBuf = path.into();

        let mut file = OpenOptions::new().read(true).open(&path)?;

        Self::parse(&mut file)
    }

    pub fn from_file_opt<P: Into<PathBuf>>(path: P) -> io::Result<Option<Self>> {
        let path: PathBuf = path.into();

        if !path.is_file() {
            return Ok(None);
        }

        Self::from_file(path).map(Some)
    }

    pub fn value<S: AsRef<str>>(&self, key: S) -> Option<&String> {
        self.kvs.get(key.as_ref()).and_then(|vs| vs.last())
    }

    pub fn values<S: AsRef<str>>(&self, key: S) -> Option<&Vec<String>> {
        self.kvs.get(key.as_ref())
    }

    pub fn value_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut String> {
        self.kvs.get_mut(key.as_ref()).and_then(|vs| vs.last_mut())
    }

    pub fn values_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut Vec<String>> {
        self.kvs.get_mut(key.as_ref())
    }

    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.kvs.contains_key(key.as_ref())
    }

    pub fn value_is_truthy<S: AsRef<str>>(value: S) -> bool {
        !matches!(value.as_ref().to_lowercase().as_str(), ""|"0"|"false"|"n")
    }

    pub fn is_truthy<S: AsRef<str>>(&self, key: S) -> bool {
        self.value(key.as_ref()).is_some_and(Self::value_is_truthy)
    }
}
