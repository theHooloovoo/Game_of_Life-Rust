// Written by theHooloovoo


pub struct Grid {
    width: usize,
    height: usize,

    pospos1: Vec<Vec<i32>>,
    pospos2: Vec<Vec<i32>>,

    state: bool,
    step: usize,
}

#[allow(dead_code)]
impl Grid {
    // Constructor
    pub fn new_blank_grid(x: usize, y: usize) -> Grid {
        Grid{width: x,
             height: y,
             pospos1: vec![vec![0; x+2]; y+2],
             pospos2: vec![vec![0; x+2]; y+2],
             state: false,
             step: 1,
        }
    }

    pub fn print_grid(&self) {
        let mut mat = &self.pospos1;
        if self.state == false { mat = &self.pospos2; } 

        for y in (1..mat.len()-1).rev() {
            let mut s = "".to_string();
            for x in 1..mat[0].len()-1 {
                // Code
                if mat[y][x] == 0 {
                    s.push('.');
                } else if mat[y][x] == 1 {
                    s.push('O');
                } else {
                    s.push('#');
                }
            }
            println!("{}", s);
        }

    }

    pub fn stamp(&mut self, data: &Vec<Vec<char>>, x_off: usize, y_off: usize) {
        // Grab the Front Buffer
        let mut mat = &mut self.pospos1;
        if self.state == false { mat = &mut self.pospos2; }

        // Check bounds!!
        
        for y in 0..data.len() {
            for x in 0..data[0].len() {
                //
                if      data[y][x] == '0' { mat[y+y_off+1][x+x_off+1] = 0; }
                else if data[y][x] == '1' { mat[y+y_off+1][x+x_off+1] = 1; }
                else                      { mat[y+y_off+1][x+x_off+1] = 2; }

            }
        }

    }


    // Add function to build from seed (text file)

    
    fn get_current_state(&mut self) -> (&mut Vec<Vec<i32>>, &mut Vec<Vec<i32>>) {
        if self.state == true {
            return (&mut self.pospos1, &mut self.pospos2);
        } else {
            return (&mut self.pospos2, &mut self.pospos1);
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
        let (mut mat, mut buf) = self.get_current_state();

        // iterate through the current matrix
        for y in 1..mat.len()-1 {
            for x in 1..mat[0].len()-1 {
                // Code here
                cell_count = 0;

                // Check each cell's neighbors to see if they are
                // living or not. Increment ceel_count if living
                // Neighbors checked are E, NE, N, NW, W, SW, S, SE
                if mat[y  ][x+1] == 1 { cell_count += 1; }   
                if mat[y+1][x+1] == 1 { cell_count += 1; }
                if mat[y+1][x  ] == 1 { cell_count += 1; }
                if mat[y+1][x-1] == 1 { cell_count += 1; }
                if mat[y  ][x-1] == 1 { cell_count += 1; }
                if mat[y-1][x-1] == 1 { cell_count += 1; }
                if mat[y-1][x  ] == 1 { cell_count += 1; }
                if mat[y-1][x+1] == 1 { cell_count += 1; }

                // Apply ruleset
                // If dead
                if mat[y][x] == 0 {
                    if cell_count == 3 { buf[y][x] = 1 } // Resserect
                    else               { buf[y][x] = 0 } // Keep dead
                } else {	// If Alive
                    if cell_count == 2 || cell_count == 3 { buf[y][x] = 1 } // Keep Alive
                    else                                  { buf[y][x] = 0 } // Kill
                }

            }
        }
        //                                                               ^^^^^^^^^^^
        } // Artificial Scope Ends. ////////////////////////////////////////////////

        self.step += 1;
        if self.state == true { self.state == false; }
        else                  { self.state == true;  }

    }
        

        
}







