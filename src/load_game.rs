// Module to add the loadGame method to the 'Game' class.

//mod battleship;
use crate::battleship::Game;

impl Game {
    /// Load the game specified by self.index into the game.
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
                self.mask[9] = 1 + 512;

                self.negativeMask[6] = 64 + 128 + 256;
                self.negativeMask[7] = 1 + 2 + 64 + 256;
                self.negativeMask[8] = 2 + 64 + 128 + 256 + 512;
                self.negativeMask[9] = 2 + 256;
            }

            4 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 2;
                self.horizontal[2] = 0;
                self.horizontal[3] = 3;
                self.horizontal[4] = 1;
                self.horizontal[5] = 4;
                self.horizontal[6] = 1;
                self.horizontal[7] = 3;
                self.horizontal[8] = 0;
                self.horizontal[9] = 4;
                self.vertical[0] = 3;
                self.vertical[1] = 2;
                self.vertical[2] = 0;
                self.vertical[3] = 1;
                self.vertical[4] = 3;
                self.vertical[5] = 2;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 4;
                self.vertical[9] = 0;

                self.mask[3] = 8;
                self.mask[7] = 128;

                self.negativeMask[0] = 2 + 4 + 512;
                self.negativeMask[1] = 4 + 512;
                self.negativeMask[2] = 4 + 8 + 16 + 512;
                self.negativeMask[3] = 4 + 16 + 512;
                self.negativeMask[4] = 4 + 8 + 16 + 512;
                self.negativeMask[5] = 4 + 16 + 512;
                self.negativeMask[6] = 4 + 64 + 128 + 256 + 512;
                self.negativeMask[7] = 4 + 64 + 512;
                self.negativeMask[8] = 4 + 64 + 128 + 256 + 512;
                self.negativeMask[9] = 4 + 512;
            }

            5 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 0;
                self.horizontal[2] = 3;
                self.horizontal[3] = 2;
                self.horizontal[4] = 1;
                self.horizontal[5] = 3;
                self.horizontal[6] = 1;
                self.horizontal[7] = 1;
                self.horizontal[8] = 1;
                self.horizontal[9] = 4;
                self.vertical[0] = 2;
                self.vertical[1] = 2;
                self.vertical[2] = 1;
                self.vertical[3] = 1;
                self.vertical[4] = 3;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 4;
                self.vertical[8] = 0;
                self.vertical[9] = 3;

                self.mask[3] = 8 + 32;
                self.mask[6] = 512;
                self.mask[7] = 512;
                self.mask[8] = 512;

                self.negativeMask[0] = 32 + 256;
                self.negativeMask[1] = 256;
                self.negativeMask[2] = 4 + 8 + 16 + 32 + 64 + 256;
                self.negativeMask[3] = 4 + 16 + 64 + 256;
                self.negativeMask[4] = 4 + 8 + 16 + 32 + 64 + 256;
                self.negativeMask[5] = 256;
                self.negativeMask[6] = 256;
                self.negativeMask[7] = 256;
                self.negativeMask[8] = 256;
                self.negativeMask[9] = 256;
            }

            6 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 5;
                self.horizontal[2] = 1;
                self.horizontal[3] = 3;
                self.horizontal[4] = 3;
                self.horizontal[5] = 0;
                self.horizontal[6] = 2;
                self.horizontal[7] = 1;
                self.horizontal[8] = 4;
                self.horizontal[9] = 0;
                self.vertical[0] = 1;
                self.vertical[1] = 4;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 0;
                self.vertical[5] = 2;
                self.vertical[6] = 2;
                self.vertical[7] = 4;
                self.vertical[8] = 0;
                self.vertical[9] = 3;

                self.mask[3] = 32 + 64;
                self.mask[8] = 1;

                self.negativeMask[2] = 16 + 32 + 64 + 128;
                self.negativeMask[3] = 128;
                self.negativeMask[4] = 16 + 32 + 64 + 128;
                self.negativeMask[7] = 1 + 2;
                self.negativeMask[8] = 2;
                self.negativeMask[9] = 1 + 2;
            }

            7 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 1;
                self.horizontal[4] = 2;
                self.horizontal[5] = 2;
                self.horizontal[6] = 0;
                self.horizontal[7] = 7;
                self.horizontal[8] = 1;
                self.horizontal[9] = 1;
                self.vertical[0] = 5;
                self.vertical[1] = 0;
                self.vertical[2] = 1;
                self.vertical[3] = 3;
                self.vertical[4] = 2;
                self.vertical[5] = 0;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 2;
                self.vertical[9] = 2;

                self.mask[2] = 256;
                self.mask[3] = 1;
                self.mask[4] = 1;
                self.mask[7] = 16;

                self.negativeMask[1] = 128 + 512;
                self.negativeMask[2] = 2;
                self.negativeMask[3] = 2 + 128 + 512;
                self.negativeMask[4] = 2;
                self.negativeMask[5] = 1 + 2;
                self.negativeMask[6] = 8 + 16 + 32;
                self.negativeMask[7] = 8 + 32;
                self.negativeMask[8] = 8 + 16 + 32;
            }

            8 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 1;
                self.horizontal[2] = 0;
                self.horizontal[3] = 1;
                self.horizontal[4] = 3;
                self.horizontal[5] = 1;
                self.horizontal[6] = 2;
                self.horizontal[7] = 0;
                self.horizontal[8] = 6;
                self.horizontal[9] = 1;
                self.vertical[0] = 2;
                self.vertical[1] = 0;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 2;
                self.vertical[6] = 2;
                self.vertical[7] = 5;
                self.vertical[8] = 1;
                self.vertical[9] = 1;

                self.mask[4] = 16 + 32 + 64;

                self.negativeMask[0] = 2;
                self.negativeMask[1] = 2;
                self.negativeMask[2] = 2;
                self.negativeMask[3] = 2 + 8 + 16 + 32 + 64 + 128;
                self.negativeMask[4] = 2 + 8 + 128;
                self.negativeMask[5] = 2 + 8 + 16 + 32 + 64 + 128;
                self.negativeMask[6] = 2;
                self.negativeMask[7] = 2;
                self.negativeMask[8] = 2 + 32;
                self.negativeMask[9] = 2;
            }

            9 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 1;
                self.horizontal[2] = 2;
                self.horizontal[3] = 2;
                self.horizontal[4] = 1;
                self.horizontal[5] = 2;
                self.horizontal[6] = 2;
                self.horizontal[7] = 6;
                self.horizontal[8] = 0;
                self.horizontal[9] = 2;
                self.vertical[0] = 6;
                self.vertical[1] = 0;
                self.vertical[2] = 3;
                self.vertical[3] = 0;
                self.vertical[4] = 2;
                self.vertical[5] = 1;
                self.vertical[6] = 1;
                self.vertical[7] = 2;
                self.vertical[8] = 3;
                self.vertical[9] = 2;

                self.mask[2] = 256;
                self.mask[3] = 256;
                self.mask[4] = 16;
                self.mask[5] = 16;

                self.negativeMask[0] = 2 + 32;
                self.negativeMask[1] = 2 + 128 + 256 + 512;
                self.negativeMask[2] = 2 + 128 + 512;
                self.negativeMask[3] = 1 + 2 + 8 + 16 + 32 + 128 + 512;
                self.negativeMask[4] = 2 + 8 + 32 + 128 + 512;
                self.negativeMask[5] = 2 + 8 + 32;
                self.negativeMask[6] = 2 + 8 + 32;
                self.negativeMask[7] = 2;
                self.negativeMask[8] = 2;
                self.negativeMask[9] = 2 + 32;
            }

            10 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 0;
                self.horizontal[1] = 4;
                self.horizontal[2] = 0;
                self.horizontal[3] = 4;
                self.horizontal[4] = 1;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 0;
                self.horizontal[8] = 5;
                self.horizontal[9] = 1;
                self.vertical[0] = 2;
                self.vertical[1] = 0;
                self.vertical[2] = 2;
                self.vertical[3] = 1;
                self.vertical[4] = 4;
                self.vertical[5] = 0;
                self.vertical[6] = 2;
                self.vertical[7] = 2;
                self.vertical[8] = 2;
                self.vertical[9] = 5;

                self.mask[5] = 256 + 512;
                self.mask[8] = 64 + 128;

                self.negativeMask[4] = 128 + 256 + 512;
                self.negativeMask[5] = 128;
                self.negativeMask[6] = 128 + 256 + 512;
                self.negativeMask[7] = 64 + 128 + 256;
                self.negativeMask[8] = 256;
                self.negativeMask[9] = 64 + 128 + 256;
            }

            11 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 0;
                self.horizontal[2] = 0;
                self.horizontal[3] = 6;
                self.horizontal[4] = 1;
                self.horizontal[5] = 2;
                self.horizontal[6] = 1;
                self.horizontal[7] = 4;
                self.horizontal[8] = 2;
                self.horizontal[9] = 3;
                self.vertical[0] = 4;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 1;
                self.vertical[4] = 1;
                self.vertical[5] = 2;
                self.vertical[6] = 2;
                self.vertical[7] = 0;
                self.vertical[8] = 1;
                self.vertical[9] = 3;

                self.mask[4] = 256;
                self.mask[5] = 16;
                self.mask[8] = 1;
                self.mask[9] = 1;

                self.negativeMask[3] = 128 + 256 + 512;
                self.negativeMask[4] = 8 + 16 + 32 + 128 + 512;
                self.negativeMask[5] = 8 + 32 + 128 + 256 + 512;
                self.negativeMask[6] = 8 + 16 + 32;
                self.negativeMask[7] = 1 + 2;
                self.negativeMask[8] = 2;
                self.negativeMask[9] = 2;
            }

            12 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 3;
                self.horizontal[2] = 3;
                self.horizontal[3] = 1;
                self.horizontal[4] = 1;
                self.horizontal[5] = 0;
                self.horizontal[6] = 1;
                self.horizontal[7] = 2;
                self.horizontal[8] = 1;
                self.horizontal[9] = 5;
                self.vertical[0] = 5;
                self.vertical[1] = 0;
                self.vertical[2] = 3;
                self.vertical[3] = 0;
                self.vertical[4] = 3;
                self.vertical[5] = 1;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 1;
                self.vertical[9] = 2;

                self.mask[6] = 0b0100000000;
                self.mask[7] = 0b0000000001;
                self.mask[8] = 0b0000000001;

                self.negativeMask[1] = 0b0000000100;
                self.negativeMask[5] = 0b1110000000;
                self.negativeMask[6] = 0b1010000010;
                self.negativeMask[7] = 0b1110000010;
                self.negativeMask[8] = 0b0000000010;
                self.negativeMask[9] = 0b0000000011;
            }

            13 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 1;
                self.horizontal[2] = 5;
                self.horizontal[3] = 1;
                self.horizontal[4] = 4;
                self.horizontal[5] = 2;
                self.horizontal[6] = 1;
                self.horizontal[7] = 0;
                self.horizontal[8] = 3;
                self.horizontal[9] = 1;
                self.vertical[0] = 3;
                self.vertical[1] = 1;
                self.vertical[2] = 1;
                self.vertical[3] = 2;
                self.vertical[4] = 2;
                self.vertical[5] = 1;
                self.vertical[6] = 1;
                self.vertical[7] = 3;
                self.vertical[8] = 0;
                self.vertical[9] = 6;

                self.mask[1] = 0b0010000000;
                self.mask[2] = 0b0010000000;
                self.mask[3] = 0b0010000000;
                self.mask[8] = 0b0000000011;

                self.negativeMask[0] = 0b0101000000;
                self.negativeMask[1] = 0b0101000000;
                self.negativeMask[2] = 0b0101000000;
                self.negativeMask[3] = 0b0101000000;
                self.negativeMask[4] = 0b0101000000;
                self.negativeMask[7] = 0b0000000111;
                self.negativeMask[8] = 0b0000000000;
                self.negativeMask[9] = 0b0000000111;
            }

            14 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 4;
                self.horizontal[2] = 1;
                self.horizontal[3] = 2;
                self.horizontal[4] = 4;
                self.horizontal[5] = 3;
                self.horizontal[6] = 3;
                self.horizontal[7] = 1;
                self.horizontal[8] = 0;
                self.horizontal[9] = 1;
                self.vertical[0] = 3;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 1;
                self.vertical[4] = 2;
                self.vertical[5] = 1;
                self.vertical[6] = 0;
                self.vertical[7] = 2;
                self.vertical[8] = 1;
                self.vertical[9] = 5;

                self.mask[3] = 0b0000001000;

                self.negativeMask[0] = 0b0000000000;
                self.negativeMask[1] = 0b1000000000;
                self.negativeMask[2] = 0b0000011100;
                self.negativeMask[3] = 0b0000010100;
                self.negativeMask[4] = 0b0000011100;
                self.negativeMask[5] = 0b0000000000;
                self.negativeMask[6] = 0b0000000000;
                self.negativeMask[7] = 0b0000000000;
                self.negativeMask[8] = 0b0000000000;
                self.negativeMask[9] = 0b0000000000;
            }

            15 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 0;
                self.horizontal[2] = 0;
                self.horizontal[3] = 2;
                self.horizontal[4] = 4;
                self.horizontal[5] = 2;
                self.horizontal[6] = 3;
                self.horizontal[7] = 1;
                self.horizontal[8] = 3;
                self.horizontal[9] = 3;
                self.vertical[0] = 2;
                self.vertical[1] = 1;
                self.vertical[2] = 3;
                self.vertical[3] = 0;
                self.vertical[4] = 1;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 2;
                self.vertical[8] = 5;
                self.vertical[9] = 2;

                self.mask[0] = 0b0000100000;
                self.mask[3] = 0b0000000011;

                self.negativeMask[0] = 0b0001010000;
                self.negativeMask[1] = 0b1001110000;
                self.negativeMask[2] = 0b0000000111;
                self.negativeMask[3] = 0b0000000000;
                self.negativeMask[4] = 0b0000000111;
                self.negativeMask[5] = 0b0000000000;
                self.negativeMask[6] = 0b0000000000;
                self.negativeMask[7] = 0b0000000000;
                self.negativeMask[8] = 0b0000000000;
                self.negativeMask[9] = 0b0000000000;
            }

            16 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 2;
                self.horizontal[2] = 4;
                self.horizontal[3] = 0;
                self.horizontal[4] = 3;
                self.horizontal[5] = 0;
                self.horizontal[6] = 4;
                self.horizontal[7] = 2;
                self.horizontal[8] = 2;
                self.horizontal[9] = 2;
                self.vertical[0] = 0;
                self.vertical[1] = 3;
                self.vertical[2] = 2;
                self.vertical[3] = 4;
                self.vertical[4] = 2;
                self.vertical[5] = 3;
                self.vertical[6] = 3;
                self.vertical[7] = 0;
                self.vertical[8] = 1;
                self.vertical[9] = 2;

                self.mask[0] = 0b1000000000;
                self.mask[1] = 0b1000000000;
                self.mask[4] = 0b0000110000;
                self.mask[7] = 0b0001100000;

                self.negativeMask[0] = 0b0100000000;
                self.negativeMask[1] = 0b0100000000;
                self.negativeMask[2] = 0b0100000000;
                self.negativeMask[3] = 0b0001111000;
                self.negativeMask[4] = 0b0000001000;
                self.negativeMask[5] = 0b0001111000;
                self.negativeMask[6] = 0b0011110000;
                self.negativeMask[7] = 0b0000010000;
                self.negativeMask[8] = 0b0011110000;
                self.negativeMask[9] = 0b0000000000;
            }

            17 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 1;
                self.horizontal[2] = 1;
                self.horizontal[3] = 0;
                self.horizontal[4] = 2;
                self.horizontal[5] = 0;
                self.horizontal[6] = 6;
                self.horizontal[7] = 0;
                self.horizontal[8] = 0;
                self.horizontal[9] = 6;
                self.vertical[0] = 2;
                self.vertical[1] = 2;
                self.vertical[2] = 1;
                self.vertical[3] = 3;
                self.vertical[4] = 0;
                self.vertical[5] = 1;
                self.vertical[6] = 3;
                self.vertical[7] = 4;
                self.vertical[8] = 4;
                self.vertical[9] = 0;

                self.mask[0] = 0b0000000000;
                self.mask[1] = 0b0000001000;
                self.mask[2] = 0b0000001000;
                self.mask[9] = 0b0001100000;

                self.negativeMask[0] = 0b0000010100;
                self.negativeMask[1] = 0b0000010100;
                self.negativeMask[2] = 0b0000010100;
                self.negativeMask[3] = 0b0000011100;
                self.negativeMask[4] = 0b0000000000;
                self.negativeMask[5] = 0b0000000000;
                self.negativeMask[6] = 0b0000000000;
                self.negativeMask[7] = 0b0000000000;
                self.negativeMask[8] = 0b0001110000;
                self.negativeMask[9] = 0b0000010000;
            }

            18 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 1;
                self.horizontal[2] = 2;
                self.horizontal[3] = 5;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 0;
                self.horizontal[7] = 1;
                self.horizontal[8] = 0;
                self.horizontal[9] = 4;
                self.vertical[0] = 5;
                self.vertical[1] = 2;
                self.vertical[2] = 2;
                self.vertical[3] = 1;
                self.vertical[4] = 2;
                self.vertical[5] = 1;
                self.vertical[6] = 1;
                self.vertical[7] = 6;
                self.vertical[8] = 0;
                self.vertical[9] = 0;

                self.mask[0] = 0b0000000010;

                self.negativeMask[0] = 0b0000000101;
                self.negativeMask[1] = 0b0000000111;
            }

            19 => {
                self.grid = 10;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 1;
                self.horizontal[2] = 0;
                self.horizontal[3] = 3;
                self.horizontal[4] = 3;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 6;
                self.horizontal[8] = 1;
                self.horizontal[9] = 1;
                self.vertical[0] = 1;
                self.vertical[1] = 1;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 5;
                self.vertical[5] = 0;
                self.vertical[6] = 7;
                self.vertical[7] = 1;
                self.vertical[8] = 5;
                self.vertical[9] = 1;

                self.mask[7] = 0b0000011000;

                self.negativeMask[6] = 0b0000011100;
                self.negativeMask[7] = 0b0000000100;
                self.negativeMask[8] = 0b0000011100;
            }

            20 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 5;
                self.horizontal[2] = 2;
                self.horizontal[3] = 2;
                self.horizontal[4] = 1;
                self.horizontal[5] = 1;
                self.horizontal[6] = 1;
                self.horizontal[7] = 1;
                self.horizontal[8] = 0;
                self.horizontal[9] = 3;
                self.vertical[0] = 1;
                self.vertical[1] = 1;
                self.vertical[2] = 2;
                self.vertical[3] = 3;
                self.vertical[4] = 1;
                self.vertical[5] = 7;
                self.vertical[6] = 1;
                self.vertical[7] = 2;
                self.vertical[8] = 0;
                self.vertical[9] = 2;

                self.negativeMask[1] = 0b0000100000;
            }

            21 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 5;
                self.horizontal[4] = 3;
                self.horizontal[5] = 3;
                self.horizontal[6] = 0;
                self.horizontal[7] = 6;
                self.vertical[0] = 5;
                self.vertical[1] = 1;
                self.vertical[2] = 2;
                self.vertical[3] = 5;
                self.vertical[4] = 2;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 6;
            }

            22 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 2;
                self.horizontal[2] = 5;
                self.horizontal[3] = 2;
                self.horizontal[4] = 1;
                self.horizontal[5] = 6;
                self.horizontal[6] = 1;
                self.horizontal[7] = 4;
                self.vertical[0] = 3;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 2;
                self.vertical[4] = 5;
                self.vertical[5] = 2;
                self.vertical[6] = 2;
                self.vertical[7] = 5;
            }

            23 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 0;
                self.horizontal[2] = 4;
                self.horizontal[3] = 2;
                self.horizontal[4] = 3;
                self.horizontal[5] = 4;
                self.horizontal[6] = 2;
                self.horizontal[7] = 5;
                self.vertical[0] = 2;
                self.vertical[1] = 4;
                self.vertical[2] = 3;
                self.vertical[3] = 4;
                self.vertical[4] = 1;
                self.vertical[5] = 5;
                self.vertical[6] = 0;
                self.vertical[7] = 6;

                self.mask[5] = 0b10000000;
                self.mask[6] = 0b10000000;
                self.negativeMask[4] = 0b01000000;
                self.negativeMask[5] = 0b01000000;
                self.negativeMask[6] = 0b01000000;
                self.negativeMask[7] = 0b11000000;
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

            25 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 0;
                self.horizontal[2] = 6;
                self.horizontal[3] = 1;
                self.horizontal[4] = 4;
                self.horizontal[5] = 1;
                self.horizontal[6] = 6;
                self.horizontal[7] = 2;
                self.vertical[0] = 4;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 4;
                self.vertical[4] = 3;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 5;

                self.mask[5] = 0b10000000;
                self.mask[6] = 0b10000000;

                self.negativeMask[4] = 0b11000000;
                self.negativeMask[5] = 0b01000000;
                self.negativeMask[6] = 0b01000000;
                self.negativeMask[7] = 0b01000000;
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

            27 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 1;
                self.horizontal[2] = 5;
                self.horizontal[3] = 2;
                self.horizontal[4] = 2;
                self.horizontal[5] = 4;
                self.horizontal[6] = 2;
                self.horizontal[7] = 4;
                self.vertical[0] = 3;
                self.vertical[1] = 3;
                self.vertical[2] = 1;
                self.vertical[3] = 6;
                self.vertical[4] = 2;
                self.vertical[5] = 4;
                self.vertical[6] = 2;
                self.vertical[7] = 4;

                self.negativeMask[4] = 64 + 128;
                self.mask[5] = 128;
                self.negativeMask[5] = 64;
                self.mask[6] = 128;
                self.negativeMask[6] = 64;
                self.negativeMask[7] = 64;
            }

            28 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 1;
                self.horizontal[2] = 3;
                self.horizontal[3] = 5;
                self.horizontal[4] = 1;
                self.horizontal[5] = 5;
                self.horizontal[6] = 1;
                self.horizontal[7] = 4;
                self.vertical[0] = 4;
                self.vertical[1] = 1;
                self.vertical[2] = 6;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 3;
                self.vertical[6] = 0;
                self.vertical[7] = 6;

                self.mask[7] = 1;
                self.negativeMask[6] = 1 + 2;
                self.negativeMask[7] = 2;
            }

            29 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 4;
                self.horizontal[2] = 2;
                self.horizontal[3] = 4;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 4;
                self.vertical[0] = 7;
                self.vertical[1] = 0;
                self.vertical[2] = 6;
                self.vertical[3] = 0;
                self.vertical[4] = 5;
                self.vertical[5] = 1;
                self.vertical[6] = 5;
                self.vertical[7] = 1;

                self.negativeMask[1] = 8 + 32;
                self.mask[2] = 16;
                self.negativeMask[3] = 8 + 32;
            }

            30 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 3;
                self.horizontal[2] = 4;
                self.horizontal[3] = 3;
                self.horizontal[4] = 3;
                self.horizontal[5] = 1;
                self.horizontal[6] = 3;
                self.horizontal[7] = 3;
                self.vertical[0] = 6;
                self.vertical[1] = 0;
                self.vertical[2] = 5;
                self.vertical[3] = 1;
                self.vertical[4] = 1;
                self.vertical[5] = 5;
                self.vertical[6] = 1;
                self.vertical[7] = 6;
            }

            31 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 1;
                self.horizontal[2] = 5;
                self.horizontal[3] = 1;
                self.horizontal[4] = 3;
                self.horizontal[5] = 2;
                self.horizontal[6] = 6;
                self.horizontal[7] = 2;
                self.vertical[0] = 3;
                self.vertical[1] = 2;
                self.vertical[2] = 4;
                self.vertical[3] = 3;
                self.vertical[4] = 2;
                self.vertical[5] = 5;
                self.vertical[6] = 0;
                self.vertical[7] = 6;
            }

            32 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 2;
                self.horizontal[2] = 4;
                self.horizontal[3] = 4;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 1;
                self.horizontal[7] = 5;
                self.vertical[0] = 5;
                self.vertical[1] = 0;
                self.vertical[2] = 6;
                self.vertical[3] = 2;
                self.vertical[4] = 1;
                self.vertical[5] = 5;
                self.vertical[6] = 1;
                self.vertical[7] = 5;
            }

            33 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 3;
                self.horizontal[4] = 1;
                self.horizontal[5] = 5;
                self.horizontal[6] = 1;
                self.horizontal[7] = 5;
                self.vertical[0] = 6;
                self.vertical[1] = 1;
                self.vertical[2] = 4;
                self.vertical[3] = 1;
                self.vertical[4] = 6;
                self.vertical[5] = 2;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
            }

            34 => {
                self.grid = 8;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 1;
                self.horizontal[2] = 3;
                self.horizontal[3] = 2;
                self.horizontal[4] = 4;
                self.horizontal[5] = 3;
                self.horizontal[6] = 3;
                self.horizontal[7] = 5;
                self.vertical[0] = 5;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 1;
                self.vertical[4] = 5;
                self.vertical[5] = 1;
                self.vertical[6] = 4;
                self.vertical[7] = 3;
            }

            35 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 1;
                self.horizontal[2] = 3;
                self.horizontal[3] = 5;
                self.horizontal[4] = 1;
                self.horizontal[5] = 4;
                self.horizontal[6] = 2;
                self.horizontal[7] = 3;
                self.horizontal[8] = 4;
                self.vertical[0] = 5;
                self.vertical[1] = 2;
                self.vertical[2] = 1;
                self.vertical[3] = 4;
                self.vertical[4] = 1;
                self.vertical[5] = 6;
                self.vertical[6] = 2;
                self.vertical[7] = 1;
                self.vertical[8] = 3;

                self.mask[4] = 1;
                self.mask[7] = 1;
                self.mask[8] = 1;

                self.negativeMask[3] = 1 + 2;
                self.negativeMask[4] = 2;
                self.negativeMask[5] = 1 + 2;
                self.negativeMask[6] = 2;
                self.negativeMask[7] = 2;
                self.negativeMask[8] = 2;
            }

            36 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 1;
                self.horizontal[2] = 4;
                self.horizontal[3] = 3;
                self.horizontal[4] = 4;
                self.horizontal[5] = 3;
                self.horizontal[6] = 0;
                self.horizontal[7] = 4;
                self.horizontal[8] = 2;
                self.vertical[0] = 3;
                self.vertical[1] = 2;
                self.vertical[2] = 6;
                self.vertical[3] = 1;
                self.vertical[4] = 1;
                self.vertical[5] = 6;
                self.vertical[6] = 1;
                self.vertical[7] = 2;
                self.vertical[8] = 3;

                self.mask[2] = 128;
                self.mask[4] = 4;
                self.mask[5] = 4;

                self.negativeMask[1] = 64 + 128 + 256;
                self.negativeMask[2] = 64 + 256;
                self.negativeMask[3] = 2 + 8 + 64 + 128 + 256;
                self.negativeMask[4] = 2 + 8;
                self.negativeMask[5] = 2 + 8;
                self.negativeMask[6] = 2 + 4 + 8;
            }

            37 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 2;
                self.horizontal[2] = 7;
                self.horizontal[3] = 1;
                self.horizontal[4] = 5;
                self.horizontal[5] = 1;
                self.horizontal[6] = 2;
                self.horizontal[7] = 4;
                self.horizontal[8] = 0;
                self.vertical[0] = 4;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 0;
                self.vertical[4] = 4;
                self.vertical[5] = 3;
                self.vertical[6] = 2;
                self.vertical[7] = 4;
                self.vertical[8] = 2;

                self.mask[0] = 16;
                self.mask[2] = 4;
                self.mask[3] = 4;
                self.mask[4] = 4;

                self.negativeMask[0] = 8 + 32;
                self.negativeMask[1] = 2 + 8 + 16 + 32;
                self.negativeMask[2] = 2 + 8;
                self.negativeMask[3] = 2 + 8;
                self.negativeMask[4] = 2 + 8;
                self.negativeMask[5] = 2 + 8;
            }

            38 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 3;
                self.horizontal[2] = 4;
                self.horizontal[3] = 2;
                self.horizontal[4] = 2;
                self.horizontal[5] = 2;
                self.horizontal[6] = 3;
                self.horizontal[7] = 2;
                self.horizontal[8] = 3;
                self.vertical[0] = 3;
                self.vertical[1] = 0;
                self.vertical[2] = 6;
                self.vertical[3] = 0;
                self.vertical[4] = 2;
                self.vertical[5] = 6;
                self.vertical[6] = 2;
                self.vertical[7] = 0;
                self.vertical[8] = 6;

                self.mask[7] = 4;
                self.mask[8] = 4 + 16;

                self.negativeMask[6] = 2 + 4 + 8;
                self.negativeMask[7] = 2 + 8 + 16 + 32;
                self.negativeMask[8] = 2 + 8 + 32;
            }

            39 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 3;
                self.horizontal[2] = 1;
                self.horizontal[3] = 6;
                self.horizontal[4] = 3;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 3;
                self.horizontal[8] = 3;
                self.vertical[0] = 6;
                self.vertical[1] = 1;
                self.vertical[2] = 5;
                self.vertical[3] = 0;
                self.vertical[4] = 3;
                self.vertical[5] = 3;
                self.vertical[6] = 1;
                self.vertical[7] = 2;
                self.vertical[8] = 4;

                self.mask[3] = 1;
                self.mask[4] = 1;
                self.mask[5] = 1;
                self.mask[8] = 16;

                self.negativeMask[2] = 2;
                self.negativeMask[3] = 2;
                self.negativeMask[4] = 2;
                self.negativeMask[5] = 2;
                self.negativeMask[6] = 2;
                self.negativeMask[7] =  8 + 16 + 32;
                self.negativeMask[8] =  8 + 32;
            }

            40 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 5;
                self.horizontal[2] = 1;
                self.horizontal[3] = 5;
                self.horizontal[4] = 1;
                self.horizontal[5] = 3;
                self.horizontal[6] = 4;
                self.horizontal[7] = 3;
                self.horizontal[8] = 2;
                self.vertical[0] = 0;
                self.vertical[1] = 5;
                self.vertical[2] = 2;
                self.vertical[3] = 3;
                self.vertical[4] = 2;
                self.vertical[5] = 6;
                self.vertical[6] = 0;
                self.vertical[7] = 4;
                self.vertical[8] = 3;

                self.mask[1] = 2 + 4;

                self.negativeMask[0] = 1 + 2 + 4 + 8 + 64;
                self.negativeMask[1] = 1 + 8 + 64;
                self.negativeMask[2] = 1 + 2 + 4 + 8 + 64;
                self.negativeMask[3] = 1 + 64;
                self.negativeMask[4] = 1 + 64;
                self.negativeMask[5] = 1 + 64;
                self.negativeMask[6] = 1 + 64;
                self.negativeMask[7] = 1 + 64;
                self.negativeMask[8] = 1 + 64;
            }

            41 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 4;
                self.horizontal[2] = 2;
                self.horizontal[3] = 4;
                self.horizontal[4] = 0;
                self.horizontal[5] = 5;
                self.horizontal[6] = 1;
                self.horizontal[7] = 6;
                self.horizontal[8] = 0;
                self.vertical[0] = 1;
                self.vertical[1] = 4;
                self.vertical[2] = 2;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 4;
                self.vertical[6] = 4;
                self.vertical[7] = 2;
                self.vertical[8] = 3;

                self.mask[0] = 4;

                self.negativeMask[0] = 2 + 8;
                self.negativeMask[1] = 2 + 4 + 8;
            }

            42 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 3;
                self.horizontal[4] = 2;
                self.horizontal[5] = 2;
                self.horizontal[6] = 4;
                self.horizontal[7] = 0;
                self.horizontal[8] = 6;
                self.vertical[0] = 0;
                self.vertical[1] = 7;
                self.vertical[2] = 1;
                self.vertical[3] = 0;
                self.vertical[4] = 7;
                self.vertical[5] = 1;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 4;

                self.mask[4] = 64;

                self.negativeMask[0] = 1 + 8;
                self.negativeMask[1] = 1 + 8;
                self.negativeMask[2] = 1 + 8;
                self.negativeMask[3] = 1 + 8 + 32 + 64 + 128;
                self.negativeMask[4] = 1 + 8 + 32 + 128;
                self.negativeMask[5] = 1 + 8 + 32 + 64 + 128;
                self.negativeMask[6] = 1 + 8;
                self.negativeMask[7] = 1 + 2 + 4 + 8 + 16 + 32 + 64 + 128 + 256;
                self.negativeMask[8] = 1 + 8;
            }

            43 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 3;
                self.horizontal[2] = 2;
                self.horizontal[3] = 0;
                self.horizontal[4] = 6;
                self.horizontal[5] = 1;
                self.horizontal[6] = 1;
                self.horizontal[7] = 2;
                self.horizontal[8] = 5;
                self.vertical[0] = 6;
                self.vertical[1] = 2;
                self.vertical[2] = 1;
                self.vertical[3] = 5;
                self.vertical[4] = 2;
                self.vertical[5] = 1;
                self.vertical[6] = 4;
                self.vertical[7] = 1;
                self.vertical[8] = 3;

                self.mask[4] = 8 + 16;

                self.negativeMask[3] = 1 + 2 + 4 + 8 + 16 + 32 + 64 + 128 + 256;
                self.negativeMask[4] = 4;
                self.negativeMask[5] = 4 + 8 + 16 + 32;
            }

            44 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 0;
                self.horizontal[2] = 5;
                self.horizontal[3] = 1;
                self.horizontal[4] = 0;
                self.horizontal[5] = 7;
                self.horizontal[6] = 2;
                self.horizontal[7] = 3;
                self.horizontal[8] = 2;
                self.vertical[0] = 3;
                self.vertical[1] = 1;
                self.vertical[2] = 3;
                self.vertical[3] = 4;
                self.vertical[4] = 3;
                self.vertical[5] = 2;
                self.vertical[6] = 2;
                self.vertical[7] = 4;
                self.vertical[8] = 3;

                self.mask[2] = 8 + 16;

                self.negativeMask[1] = 4 + 8 + 16 + 32;
                self.negativeMask[2] = 32;
                self.negativeMask[3] = 4 + 8 + 16 + 32;
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
            }

            46 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 3;
                self.horizontal[2] = 3;
                self.horizontal[3] = 4;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 3;
                self.horizontal[7] = 4;
                self.horizontal[8] = 1;
                self.vertical[0] = 3;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 5;
                self.vertical[4] = 1;
                self.vertical[5] = 1;
                self.vertical[6] = 7;
                self.vertical[7] = 1;
                self.vertical[8] = 2;
            }

            47 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 6;
                self.horizontal[2] = 0;
                self.horizontal[3] = 3;
                self.horizontal[4] = 3;
                self.horizontal[5] = 2;
                self.horizontal[6] = 5;
                self.horizontal[7] = 0;
                self.horizontal[8] = 5;
                self.vertical[0] = 0;
                self.vertical[1] = 6;
                self.vertical[2] = 1;
                self.vertical[3] = 4;
                self.vertical[4] = 1;
                self.vertical[5] = 6;
                self.vertical[6] = 2;
                self.vertical[7] = 1;
                self.vertical[8] = 4;
            }

            48 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 0;
                self.horizontal[1] = 5;
                self.horizontal[2] = 0;
                self.horizontal[3] = 6;
                self.horizontal[4] = 2;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 1;
                self.horizontal[8] = 6;
                self.vertical[0] = 1;
                self.vertical[1] = 3;
                self.vertical[2] = 3;
                self.vertical[3] = 2;
                self.vertical[4] = 2;
                self.vertical[5] = 5;
                self.vertical[6] = 2;
                self.vertical[7] = 3;
                self.vertical[8] = 4;
            }

            49 => {
                self.grid = 9;
                self.maxShip = 5;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 4;
                self.horizontal[2] = 3;
                self.horizontal[3] = 4;
                self.horizontal[4] = 2;
                self.horizontal[5] = 5;
                self.horizontal[6] = 1;
                self.horizontal[7] = 0;
                self.horizontal[8] = 4;
                self.vertical[0] = 2;
                self.vertical[1] = 1;
                self.vertical[2] = 4;
                self.vertical[3] = 3;
                self.vertical[4] = 4;
                self.vertical[5] = 1;
                self.vertical[6] = 4;
                self.vertical[7] = 3;
                self.vertical[8] = 3;
            }

            50 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 1;
                self.horizontal[2] = 6;
                self.horizontal[3] = 1;
                self.horizontal[4] = 5;
                self.horizontal[5] = 0;
                self.horizontal[6] = 0;
                self.horizontal[7] = 2;
                self.horizontal[8] = 3;
                self.horizontal[9] = 0;
                self.vertical[0] = 1;
                self.vertical[1] = 0;
                self.vertical[2] = 2;
                self.vertical[3] = 4;
                self.vertical[4] = 0;
                self.vertical[5] = 5;
                self.vertical[6] = 3;
                self.vertical[7] = 3;
                self.vertical[8] = 0;
                self.vertical[9] = 2;

                self.mask[4] = 64 + 128;

                self.negativeMask[0] = 2 + 16 + 256;
                self.negativeMask[1] = 2 + 16 + 256;
                self.negativeMask[2] = 2 + 16 + 256;
                self.negativeMask[3] = 2 + 16 + 32 + 64 + 128 + 256;
                self.negativeMask[4] = 2 + 16 + 256;
                self.negativeMask[5] = 2 + 16 + 32 + 64 + 128 + 256;
                self.negativeMask[6] = 2 + 16 + 256;
                self.negativeMask[7] = 2 + 16 + 256;
                self.negativeMask[8] = 2 + 16 + 256;
                self.negativeMask[9] = 2 + 16 + 256;
            }

            51 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 0;
                self.horizontal[2] = 1;
                self.horizontal[3] = 7;
                self.horizontal[4] = 1;
                self.horizontal[5] = 0;
                self.horizontal[6] = 1;
                self.horizontal[7] = 3;
                self.horizontal[8] = 0;
                self.horizontal[9] = 3;
                self.vertical[0] = 1;
                self.vertical[1] = 2;
                self.vertical[2] = 0;
                self.vertical[3] = 4;
                self.vertical[4] = 3;
                self.vertical[5] = 0;
                self.vertical[6] = 5;
                self.vertical[7] = 2;
                self.vertical[8] = 2;
                self.vertical[9] = 1;

                self.mask[7] = 8 + 16;

                self.negativeMask[6] = 4 + 8 + 16 + 32;
                self.negativeMask[7] = 32;
                self.negativeMask[8] = 4 + 8 + 16 + 32;
            }

            52 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 0;
                self.horizontal[1] = 0;
                self.horizontal[2] = 2;
                self.horizontal[3] = 1;
                self.horizontal[4] = 5;
                self.horizontal[5] = 3;
                self.horizontal[6] = 2;
                self.horizontal[7] = 2;
                self.horizontal[8] = 3;
                self.horizontal[9] = 2;
                self.vertical[0] = 2;
                self.vertical[1] = 1;
                self.vertical[2] = 1;
                self.vertical[3] = 5;
                self.vertical[4] = 1;
                self.vertical[5] = 5;
                self.vertical[6] = 0;
                self.vertical[7] = 3;
                self.vertical[8] = 0;
                self.vertical[9] = 2;

                self.mask[9] = 0b0000110000;

                self.negativeMask[8] = 0b0001111000;
                self.negativeMask[9] = 0b0001000000;
            }

            53 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 3;
                self.horizontal[2] = 5;
                self.horizontal[3] = 0;
                self.horizontal[4] = 1;
                self.horizontal[5] = 2;
                self.horizontal[6] = 3;
                self.horizontal[7] = 0;
                self.horizontal[8] = 5;
                self.horizontal[9] = 0;
                self.vertical[0] = 0;
                self.vertical[1] = 3;
                self.vertical[2] = 0;
                self.vertical[3] = 2;
                self.vertical[4] = 3;
                self.vertical[5] = 3;
                self.vertical[6] = 3;
                self.vertical[7] = 2;
                self.vertical[8] = 3;
                self.vertical[9] = 1;

                self.mask[4] = 0b0001000000;

                self.negativeMask[3] = 0b0011100000;
                self.negativeMask[4] = 0b0010100000;
                self.negativeMask[5] = 0b0011100000;
            }

            54 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 1;
                self.horizontal[1] = 3;
                self.horizontal[2] = 0;
                self.horizontal[3] = 1;
                self.horizontal[4] = 6;
                self.horizontal[5] = 2;
                self.horizontal[6] = 1;
                self.horizontal[7] = 1;
                self.horizontal[8] = 5;
                self.horizontal[9] = 0;
                self.vertical[0] = 0;
                self.vertical[1] = 1;
                self.vertical[2] = 3;
                self.vertical[3] = 0;
                self.vertical[4] = 3;
                self.vertical[5] = 2;
                self.vertical[6] = 4;
                self.vertical[7] = 1;
                self.vertical[8] = 2;
                self.vertical[9] = 4;

                self.mask[0] = 0b0000000100;

                self.negativeMask[0] = 0b0000001010;
                self.negativeMask[1] = 0b0000001110;
            }

            55 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 2;
                self.horizontal[1] = 5;
                self.horizontal[2] = 1;
                self.horizontal[3] = 3;
                self.horizontal[4] = 1;
                self.horizontal[5] = 3;
                self.horizontal[6] = 0;
                self.horizontal[7] = 2;
                self.horizontal[8] = 0;
                self.horizontal[9] = 3;
                self.vertical[0] = 0;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 1;
                self.vertical[4] = 3;
                self.vertical[5] = 7;
                self.vertical[6] = 0;
                self.vertical[7] = 1;
                self.vertical[8] = 2;
                self.vertical[9] = 1;

                self.mask[0] = 0b0000000010;

                self.negativeMask[0] = 0b0000000101;
                self.negativeMask[1] = 0b0000000111;
            }

            56 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 0;
                self.horizontal[1] = 2;
                self.horizontal[2] = 1;
                self.horizontal[3] = 2;
                self.horizontal[4] = 4;
                self.horizontal[5] = 3;
                self.horizontal[6] = 1;
                self.horizontal[7] = 3;
                self.horizontal[8] = 3;
                self.horizontal[9] = 1;
                self.vertical[0] = 0;
                self.vertical[1] = 2;
                self.vertical[2] = 0;
                self.vertical[3] = 6;
                self.vertical[4] = 1;
                self.vertical[5] = 2;
                self.vertical[6] = 1;
                self.vertical[7] = 1;
                self.vertical[8] = 5;
                self.vertical[9] = 2;

                self.mask[7] = 0b0000110000;

                self.negativeMask[6] = 0b0001111000;
                self.negativeMask[7] = 0b0001000000;
                self.negativeMask[8] = 0b0001111000;
            }

            57 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 4;
                self.horizontal[1] = 0;
                self.horizontal[2] = 1;
                self.horizontal[3] = 6;
                self.horizontal[4] = 2;
                self.horizontal[5] = 2;
                self.horizontal[6] = 1;
                self.horizontal[7] = 0;
                self.horizontal[8] = 3;
                self.horizontal[9] = 1;
                self.vertical[0] = 4;
                self.vertical[1] = 2;
                self.vertical[2] = 3;
                self.vertical[3] = 1;
                self.vertical[4] = 2;
                self.vertical[5] = 3;
                self.vertical[6] = 0;
                self.vertical[7] = 1;
                self.vertical[8] = 0;
                self.vertical[9] = 4;

                self.mask[3] = 0b0000010000;
                self.mask[4] = 0b0000010000;

                self.negativeMask[2] = 0b0000111000;
                self.negativeMask[3] = 0b0000101000;
                self.negativeMask[4] = 0b0000101000;
                self.negativeMask[5] = 0b0000101000;
            }

            58 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 0;
                self.horizontal[1] = 6;
                self.horizontal[2] = 2;
                self.horizontal[3] = 1;
                self.horizontal[4] = 1;
                self.horizontal[5] = 0;
                self.horizontal[6] = 2;
                self.horizontal[7] = 3;
                self.horizontal[8] = 2;
                self.horizontal[9] = 3;
                self.vertical[0] = 4;
                self.vertical[1] = 0;
                self.vertical[2] = 0;
                self.vertical[3] = 2;
                self.vertical[4] = 5;
                self.vertical[5] = 3;
                self.vertical[6] = 2;
                self.vertical[7] = 1;
                self.vertical[8] = 0;
                self.vertical[9] = 3;

                self.mask[8] = 0b1000000000;

                self.negativeMask[7] = 0b1100000000;
                self.negativeMask[8] = 0b0100000000;
                self.negativeMask[9] = 0b1100000000;
            }

            59 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 5;
                self.horizontal[1] = 0;
                self.horizontal[2] = 0;
                self.horizontal[3] = 1;
                self.horizontal[4] = 3;
                self.horizontal[5] = 3;
                self.horizontal[6] = 4;
                self.horizontal[7] = 0;
                self.horizontal[8] = 3;
                self.horizontal[9] = 1;
                self.vertical[0] = 2;
                self.vertical[1] = 0;
                self.vertical[2] = 3;
                self.vertical[3] = 2;
                self.vertical[4] = 2;
                self.vertical[5] = 2;
                self.vertical[6] = 0;
                self.vertical[7] = 6;
                self.vertical[8] = 0;
                self.vertical[9] = 3;

                self.mask[9] = 0b0000000001;

                self.negativeMask[8] = 0b0000000011;
                self.negativeMask[9] = 0b0000000010;
            }

            60 => {
                self.grid = 10;
                self.maxShip = 4;
                self.initialise();

                self.horizontal[0] = 3;
                self.horizontal[1] = 2;
                self.horizontal[2] = 3;
                self.horizontal[3] = 3;
                self.horizontal[4] = 2;
                self.horizontal[5] = 2;
                self.horizontal[6] = 1;
                self.horizontal[7] = 2;
                self.horizontal[8] = 0;
                self.horizontal[9] = 2;
                self.vertical[0] = 0;
                self.vertical[1] = 4;
                self.vertical[2] = 3;
                self.vertical[3] = 3;
                self.vertical[4] = 3;
                self.vertical[5] = 4;
                self.vertical[6] = 1;
                self.vertical[7] = 0;
                self.vertical[8] = 1;
                self.vertical[9] = 1;

                self.mask[2] = 0b1000000000;

                self.negativeMask[1] = 0b1100000000;
                self.negativeMask[2] = 0b0100000000;
                self.negativeMask[3] = 0b1100000000;
            }

        _ => {
                // This is game 2.
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

        // Add in negativeMask for any 0 in the vertical.
        let mut mask: usize = 1;
        for i in 0..self.vertical.len() {
            if self.vertical[i] == 0 {
                for j in 0..self.negativeMask.len() {
                    self.negativeMask[j] |= mask;
                }
            }
            mask *= 2;
        }

        // Add a negativeMask for 0 in the horizontal.
        // This makes no difference but the display looks better.
        for i in 0..self.horizontal.len() {
            if self.horizontal[i] == 0 {
                self.negativeMask[i] = 2_usize.pow(self.grid as u32) - 1;
            }
        }

        // Add in the user specified mask information.
        for i in 0..self.userMask.len() {
            self.mask[i] |= self.userMask[i];
        }
    }
}
