use clap::Parser;
use regex::Regex;
use std::{fs::File, io::BufReader, path::PathBuf};
pub mod command;
pub mod error;
use command::XgrepConfig;
pub use error::XgrepError;

pub struct Xgrep {
    pub config: XgrepConfig,
    pub regex: Regex,
    pub files: Vec<PathBuf>,
}

#[allow(clippy::manual_filter_map)]
impl Xgrep {
    pub fn new() -> Result<Xgrep, XgrepError> {
        let config = XgrepConfig::parse();
        Self::from(config)
    }

    pub fn from(config: XgrepConfig) -> Result<Xgrep, XgrepError> {
        let regex = Regex::new(&config.regex)?;
        let files = glob::glob(&config.glob)?
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();
        Ok(Self {
            config,
            regex,
            files,
        })
    }

    pub fn run<F>(&self, handler: &mut F) -> Result<(), XgrepError>
    where
        F: FnMut(&str, &Regex, BufReader<File>) -> Result<(), XgrepError>,
    {
        for file in &self.files {
            let f = File::open(file)?;
            if let Some(filename) = file.to_str() {
                handler(filename, &self.regex, BufReader::new(f))?;
            }
        }
        Ok(())
    }
}
