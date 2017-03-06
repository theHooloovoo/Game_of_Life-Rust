// Written by theHooloovoo

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


pub fn open_file(file_name: &str) -> Result<Vec<Vec<char>>, ()> {
    // 
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {},
    }

    let mut word = s.lines();
    let data = word.collect::<Vec<&str>>();

    // Create a Vec<Vec<char>> from the Vec<Vec<&str>>
    // Because I don't know how to do that more elegantly
    let mut char_grid: Vec<Vec<char>> = Vec::with_capacity(data.len());
    for n in 0..data.len() {
        char_grid.push(data[n].chars().collect::<Vec<char>>());
    }

    Ok(char_grid)
}
