mod colors;
mod cube;
mod initial_position;
mod moves;

use colors::Color;

pub use cube::Cube;
pub type CubeType = [[[Color; 3]; 3]; 6];
