use clap::Parser;
use regex::Regex;
use std::{fs::File, io::BufReader, path::PathBuf};
pub mod command;
pub mod error;
use command::XgrepConfig;
use error::XgrepError;

pub struct Xgrep {
    config: XgrepConfig,
    regex: Regex,
    files: Vec<PathBuf>,
}

pub type XgrepHandler<R = File> =
    fn(path: &str, regex: &Regex, reader: BufReader<R>) -> Result<(), XgrepError>;

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

    fn run(&self, handler: XgrepHandler) -> Result<(), XgrepError> {
        for file in &self.files {
            let f = File::open(file)?;
            if let Some(filename) = file.to_str() {
                handler(filename, &self.regex, BufReader::new(f))?;
            }
        }
        Ok(())
    }
}
