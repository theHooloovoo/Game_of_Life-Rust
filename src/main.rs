// Written by theHooloovoo
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// TODO:    [x] convert global variables into structs
//          [-] take command line args
//          [ ] make print_grid take args to control printing
//          [x] add function to insert sets of cells (glider, loaf, pentomino, etc)
//          [ ] parallelize set_buffer()
//          [ ] implement faster iteration techniques (i.e. only check non-empty sections)

//          [x] Add functionality to generate a Grid from a text file
//          [ ] Add functionality to generate a text file from a Grid

// Command line options
extern crate getopts;
use getopts::Options;
use std::env;

mod grid;
mod reader;

// =====================================
fn main() {
    // Declare command line arg variables
    let mut arg_file_name = "".to_string();
    let mut arg_save_bool = false;
    let mut arg_iterate_n = 0;

    // Manage command line options
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    // GENERATE OPTIONS HERE     v v v v v
    opts.optopt(  "f", "file",    "Select file to load", "Requires a valid filepath");
    opts.optopt(  "s", "save",    "Save to selected file", "Requires a valid filepath");
    opts.optflag( "q", "quiet",   "Do not print output to stdout");
    opts.optopt(  "i", "iterate", "Iterate the grid 'n' amount of times", "'n' must be a positive integer");
    // FINISH GENERATING OPTIONS ^ ^ ^ ^ ^

    // Ensure that the getopts object is good to go
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let mut main_grid = grid::Grid::new_empty_grid();

    // Configure program from command args
    if matches.opt_present("f") {
        // let input_file = matches.opt_str("f").unwrap();
        arg_file_name = matches.opt_str("f").unwrap();
        match reader::open_file(&arg_file_name) {
            Err(thing) => panic!(thing),
            Ok(data) => main_grid = grid::Grid::load_from_file_data(data),
        }
    }
    if matches.opt_present("s") {
        //let output_file = matches.opt_str("s").unwrap();
    }



    // Access a matrix such as mat[y][x]
    // let mut main_grid = grid::Grid::load_from_file(&file_name);
    // main_grid.print_grid();

}

