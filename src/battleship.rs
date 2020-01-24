// Module to define the 'Game' class.

// Member variables for the 'Game' class.
pub struct Game {
    pub index: i32,
    grid: i32,
    pub indent: i32,
    pub start: f64,
    pub finish: f64,
}

// Methods for the 'Game' class.
impl Game {
    pub fn new() -> Game {
        Game{index: 0, grid: 0, indent: 0, start: 0.0, finish: 100.0}
    }
}
