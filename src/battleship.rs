// Module to define the 'Game' class.

// Member variables for the 'Game' class.
pub struct Game {
    pub index: i32,
    pub grid: i32,
    pub indent: i32,
    pub start: f64,
    pub finish: f64,
    pub threads: i32,
    pub maxShip: i32,
    pub horizontal: Vec<i32>,
    pub vertical: Vec<i32>,
    pub mask: Vec<i32>,
    pub negativeMask: Vec<i32>,
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
        print!("\u{250F}"); // \u250F")
        for y in 0..self.grid {
            print!("\u{2501}");
        }
        println!("\u{2513}");
    }
}
