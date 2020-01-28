// Module to add the loadGame method to the 'Game' class.

//mod battleship;
use crate::battleship::Game;

impl Game {
    pub fn loadGame(&mut self) {
        match self.index {
            1 => {
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

            3 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 0;
                self.horizontal[2] = 3;
                self.horizontal[3] = 5;
                self.horizontal[4] = 2;
                self.horizontal[5] = 0;
                self.horizontal[6] = 0;
                self.horizontal[7] = 2;
                self.horizontal[8] = 2;
                self.horizontal[9] = 2;
                self.vertical[0] = 3;
                self.vertical[1] = 0;
                self.vertical[2] = 1;
                self.vertical[3] = 4;
                self.vertical[4] = 1;
                self.vertical[5] = 6;
                self.vertical[6] = 0;
                self.vertical[7] = 3;
                self.vertical[8] = 0;
                self.vertical[9] = 2;

                self.mask[7] = 128;
                self.mask[8] = 1;
                self.mask[9] = 512;

                self.negativeMask[6] = 64 + 128 + 256;
                self.negativeMask[7] = 2 + 64 + 256;
                self.negativeMask[8] = 2 + 64 + 128 + 256 + 512;
                self.negativeMask[9] = 1 + 2 + 256;
            }

            24 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 3;
                self.horizontal[2] = 4;
                self.horizontal[3] = 2;
                self.horizontal[4] = 1;
                self.horizontal[5] = 6;
                self.horizontal[6] = 0;
                self.horizontal[7] = 5;
                self.vertical[0] = 4;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 1;
                self.vertical[4] = 6;
                self.vertical[5] = 1;
                self.vertical[6] = 3;
                self.vertical[7] = 4;

                self.negativeMask[0] = 2;
                self.mask[1] = 1;
                self.negativeMask[1] = 2;
                self.mask[2] = 1;
                self.negativeMask[2] = 2;
                self.negativeMask[3] = 1+2;
            }

            26 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 2;
                self.horizontal[2] = 1;
                self.horizontal[3] = 5;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 1;
                self.horizontal[7] = 6;
                self.vertical[0] = 5;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 3;
                self.vertical[4] = 2;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 6;

                self.negativeMask[5] = 64 + 128;
                self.mask[6] = 128;
                self.negativeMask[6] = 64;
                self.mask[7] = 128;
                self.negativeMask[7] = 64;

                self.mask[0] = 128;
                self.negativeMask[1] = 64;
                self.mask[2] = 128;
                self.negativeMask[3] = 64;
                self.mask[4] = 128;
            }

            45 => {
                self.grid = 9;
                self.maxShip = 5;

                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 4;
                self.horizontal[2] = 2;
                self.horizontal[3] = 5;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 3;
                self.horizontal[7] = 1;
                self.horizontal[8] = 4;
                self.vertical[0] = 0;
                self.vertical[1] = 6;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 2;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 5;

                self.mask[0] = 0;
                self.mask[1] = 0;
                self.mask[2] = 0;
                self.mask[3] = 0;
                self.mask[4] = 0;
                self.mask[5] = 0;
                self.mask[6] = 0;
                self.mask[7] = 0;
                self.mask[8] = 0;

                self.negativeMask[0] = 0;
                self.negativeMask[1] = 0;
                self.negativeMask[2] = 0;
                self.negativeMask[3] = 0;
                self.negativeMask[4] = 0;
                self.negativeMask[5] = 0;
                self.negativeMask[6] = 0;
                self.negativeMask[7] = 0;
                self.negativeMask[8] = 0;
            }

        _ => {
                self.grid = 10;
                self.maxShip = 4;

                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 2;
                self.horizontal[4] = 3;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 1;
                self.horizontal[8] = 3;
                self.horizontal[9] = 0;
                self.vertical[0] = 1;
                self.vertical[1] = 4;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 0;
                self.vertical[6] = 1;
                self.vertical[7] = 3;
                self.vertical[8] = 2;
                self.vertical[9] = 2;

                self.mask[2] = 4 + 8 + 16;
                self.mask[3] = 128;
                self.mask[4] = 2;
                self.mask[5] = 2 + 256;
                self.mask[6] = 2;
                self.mask[7] = 2;

                self.negativeMask[0] = 2;
                self.negativeMask[1] = 2 + 4 + 8 + 16 + 32;
                self.negativeMask[2] = 2 + 32 + 128 + 256;
                self.negativeMask[3] = 2 + 4 + 8 + 16 + 32 + 256;
                self.negativeMask[4] = 128 + 256;
                self.negativeMask[5] = 128;
                self.negativeMask[6] = 128 + 256;
                self.negativeMask[8] = 2;
                self.negativeMask[9] = 2;
            }
        }

        // Add in the user specified mask information.
        for i in 0..self.userMask.len() {
            self.mask[i] |= self.userMask[i];
        }
    }
}
