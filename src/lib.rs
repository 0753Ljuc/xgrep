use clap::Parser;
use regex::Regex;
use std::{fs::File, io::BufReader, path::PathBuf};
pub mod command;
pub mod error;
use command::XgrepConfig;
pub use error::XgrepError;

pub struct Xgrep {
    config: XgrepConfig,
    regex: Regex,
    files: Vec<PathBuf>,
}

impl Xgrep {
    #[allow(clippy::manual_filter_map)]
    pub fn new() -> Result<Xgrep, XgrepError> {
        let config = XgrepConfig::parse();
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
