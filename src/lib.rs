mod colors;
mod cube;
mod initial_position;

use colors::Color;

pub use cube::Cube;
pub type CubeType = [[[Color; 3]; 3]; 6];
