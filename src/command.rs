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