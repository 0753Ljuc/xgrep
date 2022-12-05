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
    let mut writer = Vec::new();
    xgrep
        .run(&mut |a, b, c| handler(a, b, c, &mut writer))
        .unwrap();
    }

fn handler(
    path: &str,
    regex: &Regex,
    reader: BufReader<File>,
    writer: &mut impl Write,
) -> Result<(), xgrep::XgrepError> {
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
        let _i = writer
            .write(format!("{}:\n{}", path.yellow(), output_lines_vec.concat()).as_bytes())?;
            }

            Ok(())
}

fn format_line(line_id: usize, start: usize, end: usize, line_message: &str) -> String {
    format!(
        "{:>5}{:>3}:{:3}|{}{}{}\n",
        line_id,
        start.to_string().cyan(),
        end.to_string().blue(),
        &line_message[..start],
        &line_message[start..end].red(),
        &line_message[end..],
    )
}
        };

    xgrep.run(&mut handler).unwrap();
}
