fn main() {
    use regex::Regex;
    let re = Regex::new(r"(?m)^line \d+").unwrap();
    let m: Vec<_> = re.find_iter("line 1\nline 2\n").collect();
    for m in m {
        println!("Found match at {}-{}", m.start(), m.end());
        println!("Matched: {}", m.as_str());
    }
}
