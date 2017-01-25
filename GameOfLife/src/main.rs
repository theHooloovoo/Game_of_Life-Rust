// Written by theHooloovoo
#![allow(dead_code)]
#![allow(non_snake_case)]

// TODO:    convert global variables into structs
//          take command line args
//          make print_grid take args to control printing
//          add function to insert sets of cells (glider, loaf, pentomino, etc)
//          parallelize set_buffer()
//          implement faster iteration techniques (i.e. only check non-empty sections)

mod grid;
mod reader;

// =====================================
fn main() {
    // Access a matrix such as mat[y][x]

    let grid_data = reader::open_file("/home/eric/Programs/Rust/Game_of_Life-Rust/GameOfLife/resource/testFile_01");

    // let test_string = vec![vec!['1', '0', '1', '1']; 4];
    let test_string = vec![vec!['1','1','1'], vec!['1','0','0'], vec!['0','1','0']];

	let mut gridA = grid::Grid::new_blank_grid(10, 10);
    gridA.iterate();
	
    gridA.stamp(&test_string, 2, 2);

    gridA.print_grid();

    let mut matrix1 = vec![vec![false; 100+1]; 100+1];
    //let mut matrix2 = vec![vec![false; X_SIZE]; Y_SIZE];

    /*
    // Glider
    matrix1[50][50] = true;
    matrix1[50][49] = true;
    matrix1[50][48] = true;
    matrix1[51][50] = true;
    matrix1[52][49] = true;
    */

    // Lightweight Spaceship
    matrix1[51][48] = true;
    matrix1[51][49] = true;
    matrix1[51][50] = true;
    matrix1[51][51] = true;
    matrix1[50][51] = true;
    matrix1[49][51] = true;
    matrix1[48][50] = true;
    matrix1[50][47] = true;
    matrix1[48][47] = true;


    /*
    // Pentomino-O
    matrix1[5][5] = true;
    matrix1[5][6] = true;
    matrix1[5][7] = true;
    matrix1[5][8] = true;
    matrix1[5][9] = true;
    // It works!
    */

    /*
    // Pentomino-R
    matrix1[50][50] = true;
    matrix1[51][50] = true;
    matrix1[51][51] = true;
    matrix1[49][50] = true;
    matrix1[50][49] = true;
    */

    /*
    // Hexomino-Century
    matrix1[50][50] = true;
    matrix1[50][49] = true;
    matrix1[49][50] = true;
    matrix1[50][51] = true;
    matrix1[51][51] = true;
    matrix1[51][52] = true;
    */

    /*
    // Heptomino-Pi
    matrix1[50][49] = true;
    matrix1[50][50] = true;
    matrix1[50][51] = true;
    matrix1[51][49] = true;
    matrix1[51][51] = true;
    matrix1[52][49] = true;
    matrix1[52][51] = true;
    */

    // Random seed
    matrix1[50][57] = true;
    matrix1[50][58] = true;
    matrix1[49][57] = true;
    matrix1[49][58] = true;

}

