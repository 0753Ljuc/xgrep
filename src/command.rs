use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "xgrep", version = "1.0", author = "0753Ljuc")]
pub struct XgrepConfig {
    #[arg(short, long)]
    /// To match the pattern by regex
    pub regex: String,

    #[arg(short, long)]
    /// Glob of file names to search, e.g. /media/**/*.txt
    pub glob: String,
}

#[cfg(test)]
mod tests {
    use crate::command::XgrepConfig;
    use clap::Parser;

    #[test]
    fn it_work() {
        let args = vec!["xgrep", "-r", "^Pool", "-g", "*.txt"];
        let config = XgrepConfig::parse_from(&args);
        assert_eq!(config.regex, "^Pool");
        assert_eq!(config.glob, "*.txt");
    }
}
