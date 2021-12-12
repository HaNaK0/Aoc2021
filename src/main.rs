use std::{fs, io::{Error, BufReader, BufRead}};

mod util;
pub use crate::util::load_file;

mod day_2;
pub use crate::day_2::day_2;

fn main() -> Result<(), Error> {
    println!("Hello, world!");

    let data = load_file(2)?;
    day_2(data);

    Ok(())
}

#[allow(dead_code)]
fn day_1() -> Result<(), Error> {
    println!("Day 1");

    let file = fs::File::open("data/day_1.txt")?;
    let file = BufReader::new(file);
    
    let lines = file.lines();

    let line_vector : Vec<i32> = lines.map(|item| -> i32 {
        
        item.unwrap().parse::<i32>().unwrap()
    }).collect();

    let result = line_vector.iter()
        .enumerate()
        .map_while(|item| {
            if item.0 + 2 >= line_vector.len() {
                None
            } else {
                let mut sum = *item.1;
                sum += line_vector[item.0 + 1];
                sum += line_vector[item.0 + 2];
                Some((sum, 0))
            }
    }).reduce(|accum, item| {
        if accum.0 < item.0 {
            (item.0, accum.1 + 1)
        } else {
            (item.0, accum.1)
        }
    }).unwrap();

    println!("Result: {}", result.1);

    Ok(())
}

