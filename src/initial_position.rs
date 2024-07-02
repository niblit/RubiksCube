use crate::{facets::Facet, CubeType};

pub const INITIAL_POSITION: CubeType = [
    [[Facet::White; 3]; 3],
    [[Facet::Orange; 3]; 3],
    [[Facet::Green; 3]; 3],
    [[Facet::Red; 3]; 3],
    [[Facet::Blue; 3]; 3],
    [[Facet::Yellow; 3]; 3],
];
