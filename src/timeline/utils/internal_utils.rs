use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_lines(filename: &String) -> Result<Lines<BufReader<File>>, String> {
    match File::open(filename) {
        Ok(file) => Ok(BufReader::new(file).lines()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn strip_comment(line: String) -> String {
    let comment = line.find('#');
    match comment {
        Some(loc) => line.split_at(loc).0.to_string(),
        None => line,
    }
}
