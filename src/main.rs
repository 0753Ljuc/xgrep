use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

use colored::Colorize;
use regex::Regex;
use xgrep::Xgrep;

#[allow(clippy::manual_filter_map)]
fn main() {
    let xgrep = Xgrep::new().unwrap();
    let mut writer = std::io::stdout();
    fn format_line(index: usize, start: usize, end: usize, line: &str) -> String {
        format!(
            "{:>5}{:>3}:{:3}|{}{}{}",
            index,
            start.to_string().cyan(),
            end.to_string().blue(),
            &line[..start],
            &line[start..end].red(),
            &line[end..],
        )
    }
    let mut handler =
        |path: &str, regex: &Regex, reader: BufReader<File>| -> Result<(), xgrep::XgrepError> {
            let output_lines_vec = reader
                .lines()
                .filter(|l| l.is_ok())
                .enumerate()
                .map(|(index, l)| {
                    /*
                    output:    16  5:9  |stub Pool
                    or
                    output:    16  5:9  |stub Pool Pool\n    16  10:14  |stub Pool Pool
                    */
                    let line = l.unwrap();
                    regex
                        .find_iter(line.as_str())
                        .map(|m| format_line(index, m.start(), m.end(), line.as_str()))
                        .collect::<Vec<_>>()
                        .join("\n")
                })
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>();
            if !output_lines_vec.is_empty() {
                let _i = writer.write(format!("{}:\n", path.yellow()).as_bytes())?;
                let _i = writer.write(((output_lines_vec.concat() + "\n").as_str()).as_bytes())?;
            }

            Ok(())
        };

    xgrep.run(&mut handler).unwrap();
}
