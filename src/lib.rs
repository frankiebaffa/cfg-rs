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
    pub fn parse_buffered<R: Read>(contents: BufReader<R>) -> io::Result<Self> {
        let mut cfg = Config::default();

        let mut line_no = 0;
        for line_res in contents.lines() {
            let line = line_res?;

            line_no += 1;

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            if !line.contains("=") {
                return Err(io::Error::new(io::ErrorKind::Other, format!(
                    "No key-value pair found on line {line_no}."
                )));
            }

            let mut key_val_split = line.split("=");

            let key = match key_val_split.next() {
                Some(key) => key.trim(),
                None => return Err(io::Error::new(io::ErrorKind::Other, format!(
                    "No key found on line {line_no}."
                ))),
            };

            let value = key_val_split.collect::<String>();
            if !cfg.kvs.contains_key(key) {
                cfg.kvs.insert(key.to_owned(), Vec::new());
            }

            cfg.kvs.get_mut(key).unwrap().push(value.to_string());
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
        self.kvs.get(key.as_ref()).and_then(|vs| vs.first())
    }

    pub fn values<S: AsRef<str>>(&self, key: S) -> Option<&Vec<String>> {
        self.kvs.get(key.as_ref())
    }

    pub fn value_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut String> {
        self.kvs.get_mut(key.as_ref()).and_then(|vs| vs.first_mut())
    }

    pub fn values_mut<S: AsRef<str>>(&mut self, key: S) -> Option<&mut Vec<String>> {
        self.kvs.get_mut(key.as_ref())
    }

    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.kvs.contains_key(key.as_ref())
    }

    pub fn value_is_truthy<S: AsRef<str>>(value: S) -> bool {
        matches!(value.as_ref().to_lowercase().as_str(), "true"|"1"|"y")
    }

    pub fn is_truthy<S: AsRef<str>>(&self, key: S) -> bool {
        self.value(key.as_ref()).is_some_and(Self::value_is_truthy)
    }
}
