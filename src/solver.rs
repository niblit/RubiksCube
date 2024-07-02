use std::collections::HashSet;
use std::collections::VecDeque;

use super::Color;

use super::Cube;

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
            for direction in ["R", "R'", "R2", "L", "L'", "L2", "U", "U'", "U2", "D", "D'", "D2", "F", "F'", "F2", "B", "B'", "B2"] {
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

    position[0][0][1] == Color::White
    && position[0][1][0] == Color::White
    && position[0][1][1] == Color::White
    && position[0][1][2] == Color::White
    && position[0][2][1] == Color::White
    && position[1][0][1] == Color::Orange
    && position[2][0][1] == Color::Green
    && position[3][0][1] == Color::Red
    && position[4][0][1] == Color::Blue

}
