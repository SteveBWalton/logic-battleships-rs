// Module to define the 'Game' class.

// Member variables for the 'Game' class.
pub struct Game {
    pub index: usize,
    pub grid: usize,
    pub indent: usize,
    pub start: f64,
    pub finish: f64,
    pub threads: usize,
    pub maxShip: usize,
    pub horizontal: Vec<usize>,
    pub vertical: Vec<usize>,
    pub mask: Vec<usize>,
    pub negativeMask: Vec<usize>,
}

// Methods for the 'Game' class.
impl Game {
    pub fn new() -> Game {
        let horizontal = Vec::new();
        let vertical = Vec::new();
        let mask = Vec::new();
        let negativeMask = Vec::new();
        Game{index: 0, grid: 0, indent: 0, start: 0.0, finish: 100.0, threads: 1, maxShip: 0, horizontal: horizontal, vertical: vertical, mask: mask, negativeMask: negativeMask}
    }



    // Initialise the arrays.
    pub fn initialise(&mut self) {
        println!("initialise, self.grid = {}", self.grid);
        /*
        self.horizontal = vec![0; self.grid];
        self.vertical = vec![0; self.grid];
        self.mask = vec![0; self.grid];
        self.negativeMask = vec![0; self.grid];
        */
        for i in 0..self.grid {
            self.horizontal.push(0);
            self.vertical.push(0);
            self.mask.push(0);
            self.negativeMask.push(0);
        }
    }



    // Display the current board.
    pub fn displayBoard(&self) {
        println!("Logic Battleships Game Number {}", self.index);
        print!("\u{250F}");
        for y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{2513}");
        for y in 0usize..self.grid {
            print!("\u{2503}");
            for x in 0usize..self.grid {
                let mask = 2_usize.pow(x as u32);
                if self.mask[y] & mask == mask {
                    print!("\u{2588}");
                }
                else if self.negativeMask[y] & mask == mask {
                    print!("\u{00B7}");
                }
                else {
                    print!(" ");
                }
            }
            print!("\u{2503}");
            print!("{}", self.horizontal[y]);

            let lines = self.getPossibleLines(self.horizontal[y], self.mask[y], self.negativeMask[y]);
            println!("  There are {} possible lines.", lines.len());
        }
        print!("\u{2517}");
        for y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{251B}");
        print!(" ");
        for y in 0..self.grid {
            print!("{}", self.vertical[y]);
        }
        println!("");
    }



    // Returns the set of possible lines that have the specified number of solid positions.
    fn getPossibleLines(&self, numSolid :usize, mask :usize, negativeMask :usize) -> Vec<usize> {
        // print('GetPossibleLines({}, {}, {}, {}, {})'.format(nNumPositions, nNumSolid, nMaxShip, nMask, nNegativeMask))
        let mut listResult = Vec::new();
        let maximum = (2_usize.pow(self.grid as u32)) - 1;
        for pos in 0..maximum {
            if self.countSolids(pos) == numSolid {
                if pos & mask == mask {
                    if (!pos) & negativeMask == negativeMask {
                        if self.getLongestShip(pos) <= self.maxShip {
                            listResult.push(pos);
                        }
                    }
                }
            }
        }
        return listResult
    }



    // Returns the number of ship elements in the positon.
    fn countSolids(&self, position :usize) -> usize {
        let mut count :usize = 0;
        for pos in 0..self.grid {
            let mask = 2_usize.pow(pos as u32);
            if position & mask == mask {
                count += 1;
            }
        }
        return count;
    }



    // Returns the size of the longest ship in the position.
    fn getLongestShip(&self, position :usize) -> usize {
        let mut maximum :usize = 0;
        let mut current :usize = 0;
        for pos in 0..self.grid {
            let mask = 2_usize.pow(pos as u32);
            if position & mask == mask {
                current += 1;
                if current > maximum {
                    maximum = current;
                }
            }
            else {
                current = 0;
            }
        }
        return maximum;
    }
}

