use std::{io::{BufReader, Error, Read, BufRead}, fs::{File}};

pub fn load_file<R: Read>(day: usize) -> Result<Vec<String>, Error>{
    let path = "data/day_".to_string() + &day.to_string() + ".txt";

    let file = File::open(path)?;
    let file = BufReader::new(file);

    return file.lines().collect();
}