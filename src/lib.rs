mod colors;
mod cube;
mod initial_position;
mod moves;
mod solver;

use colors::Color;

pub use cube::Cube;
pub use solver::solve;

pub type CubeType = [[[Color; 3]; 3]; 6];
