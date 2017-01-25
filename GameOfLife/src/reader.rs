// Written by theHooloovoo

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Reader {
    //
    fileName: String,

}

pub fn open_file(file_name: &str) /* -> Vec<Vec<char>> */ {
    // 
    let path = Path::new(file_name);
    let display = path.display();

    let mut file = match File::open(&path) {

        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => {},
    }

    let mut word = s.lines();
    let data = word.collect::<Vec<&str>>();

    let mut grid_data = vec![vec!['0'; data[0].len()]; data.len()];

    let mut y = 0;
    for height in data {
        let mut x = 0;
        for width in height.chars() {
            // code
            grid_data[y][x] = width;
            x += 1;
        }
        y += 1;
    }

    for y in grid_data {
        let mut q = "".to_string();
        for x in y {
            q.push(x);
        }
        println!("{}", q);
    }

}
