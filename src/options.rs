// Written by theHooloovoo

// Command line options
extern crate getopts;
use getopts::Options;
use std::env;

// -f --file [filepath]    Select file to load
// -s --save [filepath]    Save to selected file
// -i --iterate [n]   Iterate n times (n is an integer)
// -w --wrap   Allow grid to wrap around.
// -g --grow   Allow grid to grow in size. 

fn parse_commands(file_name: &str) {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.reqopt("f", "file", "Select file to load", "File that will be iterated over");
    opts.optflag("s", "save", "Save to selected file");

}
