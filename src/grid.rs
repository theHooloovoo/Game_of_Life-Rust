// Written by theHooloovoo


pub struct Grid {
    width: usize,
    height: usize,

    pospos1: Vec<Vec<char>>,
    pospos2: Vec<Vec<char>>,

    state: bool,
    step: usize,
}

#[allow(dead_code)]
impl Grid {
    // Constructor
    pub fn new_blank_grid(x: usize, y: usize) -> Grid {
        Grid{width: x,
             height: y,
             pospos1: vec![vec!['0'; x+2]; y+2],
             pospos2: vec![vec!['0'; x+2]; y+2],
             state: false,
             step: 0,
        }
    }
    pub fn load_from_file_data(file: Vec<Vec<char>>) -> Grid {
        // Create a blank grid
        let mut g = Grid::new_blank_grid(file.len(), file[0].len());

        // Stamp file data onto blank Grid
        g.stamp(&file, 0, 0);

        g  // Return this
    }

    pub fn new_empty_grid() -> Grid {
        Grid::new_blank_grid(0, 0)
    }

    pub fn print_grid(&self) {
        let mut mat = &self.pospos1;
        if self.state == false { mat = &self.pospos2; } 

        for y in (1..mat.len()-1).rev() {
            let mut s = "".to_string();
            for x in 1..mat[0].len()-1 {
                // Code
                if mat[y][x] == '0' {
                    s.push('0');
                } else if mat[y][x] == '1' {
                    s.push('1');
                } else {
                    s.push('?');
                }
            }
            println!("{}", s);
        }

    }

    pub fn grow_left(&mut self, n: usize) {
        //
    }
    pub fn grow_right(&mut self, n: usize) {
        //
    }
    pub fn grow_upwards(&mut self, n: usize) {
        //
    }
    pub fn grow_downwards(&mut self, n: usize) {
        //
    }
    
    pub fn stamp(&mut self, data: &Vec<Vec<char>>, x_off: usize, y_off: usize) {
        // Grab the Front Buffer
        let mut mat = &mut self.pospos1;
        if self.state == false { mat = &mut self.pospos2; }

        // Check bounds!!
        
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                //
                if      data[y][x] == '0' { mat[y+y_off+1][x+x_off+1] = '0'; }
                else if data[y][x] == '1' { mat[y+y_off+1][x+x_off+1] = '1'; }
                else                      { mat[y+y_off+1][x+x_off+1] = '2'; }

            }
        }

    }

    // Add function to build from seed (text file)

    fn get_current_state(&mut self) -> (&Vec<Vec<char>>, &mut Vec<Vec<char>>) {
        if self.state == true {
            return (&self.pospos1, &mut self.pospos2);
        } else {
            return (&self.pospos2, &mut self.pospos1);
        }
    }
    
    // Loops through the grid, generating values for the next step
    pub fn iterate(&mut self) {
        // 
        let mut cell_count = 0;

        { // Artificial Scope used to kill mat and buf /////////////////////////////
          // so that &mut self can modify itself                         vvvvvvvvvvv

        // Make references to easily refer to the
        // current matrix and buffer
        let (front, mut back) = self.get_current_state();

        // iterate through the current matrix
        for y in 1..front.len()-1 {
            for x in 1..front[0].len()-1 {
                // Code here
                cell_count = 0;

                // Check each cell's neighbors to see if they are
                // living or not. Increment cell_count if living
                // Neighbors checked are E, NE, N, NW, W, SW, S, SE
                if front[y  ][x+1] == '1' { cell_count += 1; }   
                if front[y+1][x+1] == '1' { cell_count += 1; }
                if front[y+1][x  ] == '1' { cell_count += 1; }
                if front[y+1][x-1] == '1' { cell_count += 1; }
                if front[y  ][x-1] == '1' { cell_count += 1; }
                if front[y-1][x-1] == '1' { cell_count += 1; }
                if front[y-1][x  ] == '1' { cell_count += 1; }
                if front[y-1][x+1] == '1' { cell_count += 1; }

                // Apply ruleset
                // If dead
                if front[y][x] == '0' {
                    if cell_count == 3 { back[y][x] = '1' } // Resserect
                    else               { back[y][x] = '1' } // Keep dead
                } else {	// If Alive
                    if cell_count == 2 || cell_count == 3 { back[y][x] = '1' } // Keep Alive
                    else                                  { back[y][x] = '1' } // Kill
                }

            }
        }
        //                                                               ^^^^^^^^^^^
        } // Artificial Scope Ends. ////////////////////////////////////////////////

        self.step += 1;
        if self.state == true { self.state == false; }
        else                  { self.state == true;  }

    }
        
}   // end impl

