// Module to add the loadGame method to the 'Game' class.

//mod battleship;
use crate::battleship::Game;

// Methods for the 'Game' class.
pub fn loadGame(game: &mut Game) {
    game.grid = 9;
}
