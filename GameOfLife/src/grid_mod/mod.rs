// Written by theHooloovoo

// mod grid_mod {

	pub fn hello() {
		println!("Hello Asshole.");
	}

	pub struct Grid {
		width: usize,
		height: usize,

		pospos1: Vec<Vec<i32>>,
		pospos2: Vec<Vec<i32>>,

		state: bool,
		step: usize,
	}

	impl Grid {
		// Constructor
		pub fn new_blank_grid(x: usize, y: usize) -> Grid {
			Grid{width: x,
			     height: y,
			     pospos1: vec![vec![0; x+1]; y+1],
			     pospos2: vec![vec![0; x+1]; y+1],
				 state: false,
				 step: 1,
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
		
		fn iterate(&mut self) {
			// 
			let mut cell_count = 0;

			{ // Artificial Scope used to kill mat and buf
			  // so that &mut self can modify itself

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
					if mat[y  ][x+1] == 1 { cell_count = cell_count + 1; }   
					if mat[y+1][x+1] == 1 { cell_count = cell_count + 1; }
					if mat[y+1][x  ] == 1 { cell_count = cell_count + 1; }
					if mat[y+1][x-1] == 1 { cell_count = cell_count + 1; }
					if mat[y  ][x-1] == 1 { cell_count = cell_count + 1; }
					if mat[y-1][x-1] == 1 { cell_count = cell_count + 1; }
					if mat[y-1][x  ] == 1 { cell_count = cell_count + 1; }
					if mat[y-1][x+1] == 1 { cell_count = cell_count + 1; }

					// Apply ruleset
					// If dead
					if mat[y][x] == 0 {
						if cell_count == 3 { buf[y][x] = 1 }
						else               { buf[y][x] = 0 }
					} else {	
						if cell_count == 2 || cell_count == 3 { buf[y][x] = 1 }
						else                                  { buf[y][x] = 0 }
					}

				}
			}
			} // Artificial Scope Ends.

			self.step += 1;
			if self.state == true { self.state == false; }
			else                  { self.state == true;  }

		}
			

			
	}



// }




