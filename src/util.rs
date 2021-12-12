use std::{io::{BufReader, Error, BufRead}, fs::File};

pub fn load_file(day: usize) -> Result<Vec<String>, Error>{
    let path = "data/day_".to_string() + &day.to_string() + ".txt";

    let file = File::open(path)?;
    let file = BufReader::new(file);

    file.lines().collect()
}