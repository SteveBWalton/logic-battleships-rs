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
        self.mask[0] = 2_i32.pow(8);
        self.negativeMask[2] = 1 + 2 + 8 + 16 + 32 + 128;
        self.mask[3] = 64;
        self.mask[4] = 2 + 64 + 256 + 512;
    }
}

// Methods for the 'Game' class.
pub fn loadGame(game: &mut Game) {
    game.grid = 10;
    game.maxShip = 4;

    game.initialise();

    game.horizontal[0] = 1;
    game.horizontal[1] = 1;
    game.horizontal[2] = 3;
    game.horizontal[3] = 1;
    game.horizontal[4] = 4;
    game.horizontal[5] = 0;
    game.horizontal[6] = 0;
    game.horizontal[7] = 8;
    game.horizontal[8] = 1;
    game.horizontal[9] = 1;

    game.vertical[0] = 2;
    game.vertical[1] = 1;
    game.vertical[2] = 3;
    game.vertical[3] = 1;
    game.vertical[4] = 1;
    game.vertical[5] = 1;
    game.vertical[6] = 3;
    game.vertical[7] = 1;
    game.vertical[8] = 5;
    game.vertical[9] = 2;

    game.mask[1] = 4;
    game.mask[4] = 2;
    game.mask[7] = 16;

    // Extra information.
    game.mask[0] = 2_i32.pow(8);
    game.negativeMask[2] = 1 + 2 + 8 + 16 + 32 + 128;
    game.mask[3] = 64;
    game.mask[4] = 2 + 64 + 256 + 512;
}
