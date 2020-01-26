// Module to add the loadGame method to the 'Game' class.

//mod battleship;
use crate::battleship::Game;

impl Game {
    pub fn loadGame(&mut self) {
        self.grid = 10;
        self.maxShip = 4;

        self.initialise();

        self.horizontal[0] = 1;
        self.horizontal[1] = 1;
        self.horizontal[2] = 3;
        self.horizontal[3] = 1;
        self.horizontal[4] = 4;
        self.horizontal[5] = 0;
        self.horizontal[6] = 0;
        self.horizontal[7] = 8;
        self.horizontal[8] = 1;
        self.horizontal[9] = 1;

        self.vertical[0] = 2;
        self.vertical[1] = 1;
        self.vertical[2] = 3;
        self.vertical[3] = 1;
        self.vertical[4] = 1;
        self.vertical[5] = 1;
        self.vertical[6] = 3;
        self.vertical[7] = 1;
        self.vertical[8] = 5;
        self.vertical[9] = 2;

        self.mask[1] = 4;
        self.mask[4] = 2;
        self.mask[7] = 16;

        // Extra information.
        self.mask[0] = 2_usize.pow(8);
        self.negativeMask[2] = 1 + 2 + 8 + 16 + 32 + 128;
        self.mask[3] = 64;
        self.mask[4] = 2 + 64 + 256 + 512;
    }
}
