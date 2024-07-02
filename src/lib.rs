mod cube;
mod facets;
mod initial_position;
mod moves;
mod solver;

pub type CubeType = [[[facets::Facet; 3]; 3]; 6];

pub mod prelude {
    pub use crate::cube::Cube;
    pub use crate::solver::solve;
}
