#[cfg(test)]
mod test;

use std::{
    collections::HashMap,
    error::Error as StdError,
    fmt::{ Display, Formatter, Result as FmtResult, },
    fs::OpenOptions,
    io::{ Error as IOError, Read, },
    path::PathBuf,
    result::Result as StdResult,
};

#[derive(Debug)]
pub enum CfgError {
    IOError(IOError),
    ParseError(String),
}

impl Display for CfgError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Self::IOError(e) => e.fmt(f),
            Self::ParseError(e) => e.fmt(f),
        }
    }
}

impl StdError for CfgError {}

pub type Result<T> = StdResult<T, CfgError>;

pub type Cfg = HashMap<String, String>;

pub trait FromCfg {
    fn parse(contents: String) -> Result<Cfg>;
    fn from_file<P: Into<PathBuf>>(path: P) -> Result<Cfg>;
}

impl FromCfg for Cfg {
    fn parse(contents: String) -> Result<Cfg> {
        let mut lines = contents.lines();

        let mut cfg = Cfg::new();

        let mut line_no = 0;

        loop {
            line_no += 1;

            let line = match lines.next() {
                Some(line) => line,
                None => break,
            };

            if line.trim().is_empty() {
                continue;
            } else if &line[0..2] == "//" {
                continue;
            }

            if !line.contains("=") {
                return Err(CfgError::ParseError(format!(
                    "No key-value pair found on line {line_no}."
                )));
            }

            let mut key_val_split = line.split("=");

            let key = match key_val_split.next() {
                Some(key) => key.trim(),
                None => return Err(CfgError::ParseError(format!(
                    "No key found on line {line_no}."
                ))),
            };

            let rest = key_val_split.collect::<String>();

            let mut value = rest.trim();

            if value.starts_with("\"") && value.ends_with("\"") {
                value = &value[1..value.len()-1];
            }

            cfg.insert(key.to_string(), value.to_string());
        }

        Ok(cfg)
    }

    fn from_file<P: Into<PathBuf>>(path: P) -> Result<Cfg> {
        let path: PathBuf = path.into();

        let mut file = OpenOptions::new().read(true).open(&path)
            .map_err(|e| CfgError::IOError(e))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|e| CfgError::IOError(e))?;

        Self::parse(contents)
    }
}
