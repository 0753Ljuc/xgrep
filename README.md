```shell
Usage: xgrep.exe --regex <REGEX> --glob <GLOB>

Options:
  -r, --regex <REGEX>  To match the pattern by regex
  -g, --glob <GLOB>    Glob of file names to search, e.g. /media/**/*.txt
  -h, --help           Print help information
  -V, --version        Print version information
```

**Example**:
```shell
cargo run -- -r Pool -g stub/**/*.txt
```
