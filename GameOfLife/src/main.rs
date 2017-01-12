// Written by theHooloovoo

// TODO:    convert global variables into structs
//          take command line args
//          make print_grid take args to control printing
//          add function to insert sets of cells (glider, loaf, pentomino, etc)
//          parallelize set_buffer()
//          implement faster iteration techniques (i.e. only check non-empty sections)

// Size of Matrix
const X_SIZE: usize = 100;
const Y_SIZE: usize = 100;
const MATRIX_BOOL: bool = true;

const STEP_LIMIT: usize = 200;

// const useMat1: bool = true;

// Define a struct to contain the whole grid object
struct Matrix {
    x: usize,
    y: usize,

    is_1: bool,

    mat_1: Vec<Vec<bool>>,
    mat_2: Vec<Vec<bool>>,
}

// Used to print the contents of the current matrix
fn print_grid(grid: &Vec<Vec<bool>>) {
    // More advanced
    let mut str = "".to_string();
    // print from the bottom up so that (0,0) is at bottom left corner
    for y in (1..grid.len()-1).rev() {
        for x in 1..grid[0].len()-1 {
            if grid[y][x] == true {
                str.push_str("o");
            } else {
                str.push_str("'");
            }

        }
        println!("{}", str);
        str.clear();
    }
}

// Modify the second buffer with the data from the matrix
fn set_buffer(matrix: &Vec<Vec<bool>>, buffer: &mut Vec<Vec<bool>>) {
    // For each cell determine the state of the buffer cell
    // based on the matrix cell's neighbors.
    let mut cell_count = 0;

    // Ignore the edges of the matrices
    // (I'm lazy and don't want to deal with edge cases)
    for y in 1..matrix.len()-1 {
        for x in 1..matrix[0].len()-1 {
            // Reset cell_count
            cell_count = 0;
            // Check through body 
            if matrix[y  ][x+1] == true { cell_count = cell_count + 1; }   
            if matrix[y+1][x+1] == true { cell_count = cell_count + 1; }
            if matrix[y+1][x  ] == true { cell_count = cell_count + 1; }
            if matrix[y+1][x-1] == true { cell_count = cell_count + 1; }
            if matrix[y  ][x-1] == true { cell_count = cell_count + 1; }
            if matrix[y-1][x-1] == true { cell_count = cell_count + 1; }
            if matrix[y-1][x  ] == true { cell_count = cell_count + 1; }
            if matrix[y-1][x+1] == true { cell_count = cell_count + 1; }

            // If dead
            if matrix[y][x] == false {
                if cell_count == 3 { buffer[y][x] = true }
                else               { buffer[y][x] = false }
            // If alive
            } else {
                if cell_count == 2 || cell_count == 3 { buffer[y][x] = true }
                else                                  { buffer[y][x] = false }
            }

        }
    }
}

fn main() {
    // Access a matrix such as mat[y][x]

    let mut matrix1 = vec![vec![false; X_SIZE+1]; Y_SIZE+1];
    let mut matrix2 = vec![vec![false; X_SIZE+1]; Y_SIZE+1];
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

    // Orint current Seed
    print_grid(&matrix1);


    println!("{}", " ------------------------- ");
    for n in 1..STEP_LIMIT+1 {
        if n % 2 == 0 { set_buffer(&matrix2, &mut matrix1); }
        else          { set_buffer(&matrix1, &mut matrix2); }
    }
    if STEP_LIMIT % 2 == 0 { print_grid(&matrix1); }
    else                   { print_grid(&matrix2); }
    
    println!("Step: {}", STEP_LIMIT);
    

    // println!("{}", " ---------- ");
    // print_grid(&matrix2);

}

