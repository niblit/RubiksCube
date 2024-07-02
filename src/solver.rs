use std::cmp::max;
use std::collections::HashSet;
use std::collections::VecDeque;

use crate::cube::Cube;
use crate::facets::Facet;
use crate::moves::Moves;
use crate::CubeType;

pub fn solve(cube: &Cube) -> Vec<Moves> {
    search(cube, cross_heuristics)
}

#[derive(Clone)]
struct Node {
    moves: Vec<Moves>,
    score: u8,
}

fn search(start: &Cube, state_evaluation_fn: impl Fn(&Cube) -> u8) -> Vec<Moves> {
    let mut cube = start.clone();
    cube.clear_past_moves();

    let mut visited: HashSet<CubeType> = HashSet::new();
    let mut queue: VecDeque<Node> = VecDeque::from([Node {
        moves: Vec::new(),
        score: state_evaluation_fn(&cube),
    }]);

    loop {
        let current_node = queue.pop_front().unwrap();

        if current_node.score == u8::MAX {
            return current_node.moves;
        }

        for to_move in &current_node.moves {
            cube.make_move(*to_move);
        }

        if !visited.contains(&cube.get_pieces()) {
            visited.insert(cube.get_pieces());

            let mut nodes: Vec<Node> = Vec::new();

            for direction in Moves::iterate() {
                cube.make_move(*direction);

                let node = Node {
                    moves: cube.get_moves(),
                    score: state_evaluation_fn(&cube),
                };
                nodes.push(node);

                cube.undo_move();
            }

            let mut max_score = 0;
            for node in &nodes {
                if node.score > max_score {
                    max_score = node.score
                }
            }

            for node in nodes {
                if node.score >= max_score {
                    queue.push_back(node)
                }
            }
        }

        for _ in 0..current_node.moves.len() {
            cube.undo_move();
        }

        if queue.is_empty() {
            panic!("Could not find a solution, something is wrong");
        }
    }
}

pub fn cross_heuristics(cube: &Cube) -> u8 {
    let position = cube.get_pieces();

    let heuristics = [
        position[0][0][1] == Facet::White,
        position[0][1][0] == Facet::White,
        position[0][1][1] == Facet::White,
        position[0][1][2] == Facet::White,
        position[0][2][1] == Facet::White,
        position[1][0][1] == Facet::Orange,
        position[2][0][1] == Facet::Green,
        position[3][0][1] == Facet::Red,
        position[4][0][1] == Facet::Blue,
    ];

    let sum = heuristics.iter().filter(|&&value| value).count() as u8;

    if sum == 9 {
        u8::MAX
    } else {
        sum
    }
}
