use std::collections::HashSet;
use std::collections::VecDeque;

use crate::facets::Facet;
use crate::cube::Cube;

pub fn solve(cube: &Cube) -> Cube {
    let mut queue = VecDeque::new();
    queue.push_back(cube.clone());

    let mut visited = HashSet::new();

    let mut current;
    loop {
        current = queue.pop_front().unwrap();
        visited.insert(current.get_pieces());

        if is_cross_solved(&current) {
            break;
        } else {
            for direction in [
                "R", "R'", "R2", "L", "L'", "L2", "U", "U'", "U2", "D", "D'", "D2", "F", "F'",
                "F2", "B", "B'", "B2",
            ] {
                let mut new_cube = current.clone();
                new_cube.scramble(direction);
                if !visited.contains(&new_cube.get_pieces()) {
                    queue.push_back(new_cube);
                }
            }
        }

        if queue.is_empty() {
            break;
        }
    }

    current.clone()
}

pub fn is_cross_solved(cube: &Cube) -> bool {
    let position = cube.get_pieces();

    position[0][0][1] == Facet::White
        && position[0][1][0] == Facet::White
        && position[0][1][1] == Facet::White
        && position[0][1][2] == Facet::White
        && position[0][2][1] == Facet::White
        && position[1][0][1] == Facet::Orange
        && position[2][0][1] == Facet::Green
        && position[3][0][1] == Facet::Red
        && position[4][0][1] == Facet::Blue
}
