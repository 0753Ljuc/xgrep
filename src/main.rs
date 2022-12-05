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

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{BufReader, Write},
    };

    use colored::Colorize;
    use regex::Regex;
    use xgrep::XgrepError;

    use crate::{format_line, handler};

    #[test]
    fn test_format_line() {
        use crate::format_line;

        assert_eq!(
            format_line(8, 0, 5, "Hello I am a line"),
            format!(
                "{:>5}{:>3}:{:3}|{}{}{}\n",
                8,
                "0".cyan(),
                "5".blue(),
                "",
                "Hello".red(),
                " I am a line",
            )
        )
    }

    #[test]
    fn test_handler() -> Result<(), XgrepError> {
        // stub file
        let file_name = "test.txt";
        let mut f = File::create(file_name)?;
        f.write_all(b"\nHello I am a line \nHere is the second line \nline end")?;

        // test 1
        let mut writer = Vec::new();
        handler(
            file_name,
            &Regex::new("^line")?,
            BufReader::new(File::open(file_name)?),
            &mut writer,
        )?;
        assert_eq!(
            String::from_utf8(writer).unwrap(),
            format!(
                "{}:\n{}",
                file_name.yellow(),
                format_line(3, 0, 4, "line end")
            )
        );

        // test 2
        let mut writer = Vec::new();
        handler(
            file_name,
            &Regex::new("line")?,
            BufReader::new(File::open(file_name)?),
            &mut writer,
        )?;
        assert_eq!(
            String::from_utf8(writer).unwrap(),
            format!(
                "{}:\n{}",
                file_name.yellow(),
                format_line(1, 13, 17, "Hello I am a line ")
                    + &format_line(2, 19, 23, "Here is the second line ")
                    + &format_line(3, 0, 4, "line end")
            )
        );

        // test 3
        let mut writer = Vec::new();
        handler(
            file_name,
            &Regex::new("none")?,
            BufReader::new(File::open(file_name)?),
            &mut writer,
        )?;
        assert_eq!(String::from_utf8(writer).unwrap().trim(), "",);

        // remove stub file
        fs::remove_file(file_name).unwrap();
        Ok(())
    }
}
