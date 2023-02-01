use std::fs::File;
use std::io::{ self, BufRead, BufReader };

pub fn read_lines(filename: String) -> Result<io::Lines<BufReader<File>>, String> {
    match File::open(filename) {
        Ok(file) => Ok(io::BufReader::new(file).lines()),
        Err(e) => Err(e.to_string())
    }
}

pub fn strip_comment(line: String) -> String {
    let comment = line.find('#');
    match comment {
        Some(loc) => {
            line.split_at(loc).0.to_string()
        },
        None => line
    }
}