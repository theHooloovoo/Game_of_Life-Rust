// Written by theHooloovoo

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn open_file(file_name: &str) -> Result<Vec<Vec<char>>, &'static str> {
    // Create a Path object to open up
    let path = Path::new(file_name);
    // let display = path.display();

    // Check if file exists, then open it (Or return early)
    let mut file = match File::open(&path) {
        Err(why) => return Err("Couldn't open the fucking file"),
        Ok(file) => file,
    };
    
    // Check if file is readable, then read from it (Or return early)
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => return Err("Couldn't write to file"),
        Ok(_) => {},
    }

    // Convert file contents into string
    let mut word = s.lines();
    // Then split that string by each line ending
    let data = word.collect::<Vec<&str>>();
    // Then convert that into a grid of char
    let mut char_grid: Vec<Vec<char>> = Vec::with_capacity(data.len());
    for n in 0..data.len() {
        char_grid.push(data[n].chars().collect::<Vec<char>>());
    }
    
    // Add code to detect if all the sub Vecs are the same length!

    Ok(char_grid)
}
