use std::fmt;

use crate::{facets::Facet, initial_position::INITIAL_POSITION, moves::Moves, CubeType};

use colored::{ColoredString, Colorize};

#[derive(Clone)]
pub struct Cube {
    pieces: CubeType,
    moves: Vec<Moves>,
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn piece_to_square(cube: &Cube, face: usize, row: usize, col: usize) -> ColoredString {
            let color = cube.pieces[face][row][col];
            match color {
                Facet::White => {
                    if row == 1 && col == 1 { " ▪ " } else { " W " }.on_truecolor(255, 255, 255)
                }
                Facet::Orange => {
                    if row == 1 && col == 1 { " ▪ " } else { " O " }.on_truecolor(255, 165, 0)
                }
                Facet::Green => {
                    if row == 1 && col == 1 { " ▪ " } else { " G " }.on_truecolor(0, 255, 0)
                }
                Facet::Red => {
                    if row == 1 && col == 1 { " ▪ " } else { " R " }.on_truecolor(255, 0, 0)
                }
                Facet::Blue => {
                    if row == 1 && col == 1 { " ▪ " } else { " B " }.on_truecolor(0, 0, 255)
                }
                Facet::Yellow => {
                    if row == 1 && col == 1 { " ▪ " } else { " Y " }.on_truecolor(255, 255, 0)
                }
            }
            .truecolor(0, 0, 0)
        }

        let mut top_face = String::new();
        for row in 0..3 {
            top_face = format!("{top_face}         ");
            for col in 0..3 {
                top_face = format!("{top_face}{}", piece_to_square(self, 0, row, col));
            }
            top_face = format!("{top_face}\n");
        }

        let mut mid_row = String::new();
        for row in 0..3 {
            for face in 1..=4 {
                for col in 0..3 {
                    mid_row = format!("{mid_row}{}", piece_to_square(self, face, row, col));
                }
            }
            mid_row = format!("{mid_row}\n");
        }

        let mut bottom_face = String::new();
        for row in 0..3 {
            bottom_face = format!("{bottom_face}         ");
            for col in 0..3 {
                bottom_face = format!("{bottom_face}{}", piece_to_square(self, 5, row, col));
            }
            bottom_face = format!("{bottom_face}\n");
        }
        write!(f, "{}{}{}", top_face, mid_row, bottom_face)
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            pieces: INITIAL_POSITION,
            moves: Vec::new(),
        }
    }
}

impl Cube {
    pub fn get_pieces(&self) -> CubeType {
        self.pieces
    }

    pub fn clear_past_moves(&mut self) {
        self.moves.clear();
    }

    pub fn moves_as_string(&self) -> String {
        self.moves
            .iter()
            .map(|item| item.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn scramble(&mut self, scramble: &str) -> bool {
        let scramble = scramble.split(' ').collect::<Vec<&str>>();

        let mut scramble_moves = Vec::new();
        for scramble_move in scramble {
            scramble_moves.push(Moves::from_notation(scramble_move));
        }

        let are_all_moves_valid = !scramble_moves.contains(&None);

        if are_all_moves_valid {
            let mut pending_moves = Vec::new();
            for pending_move in scramble_moves {
                pending_moves.push(pending_move.unwrap());
            }

            for r#move in pending_moves {
                self.make_move(r#move);
            }
            true
        } else {
            false
        }
    }

    pub fn make_move(&mut self, to_move: Moves) {
        self.moves.push(to_move);

        match to_move {
            Moves::Up => self.up(),
            Moves::UpPrime => self.up_prime(),
            Moves::Up2 => self.up_2(),
            Moves::Down => self.down(),
            Moves::DownPrime => self.down_prime(),
            Moves::Down2 => self.down_2(),
            Moves::Right => self.right(),
            Moves::RightPrime => self.right_prime(),
            Moves::Right2 => self.right_2(),
            Moves::Left => self.left(),
            Moves::LeftPrime => self.left_prime(),
            Moves::Left2 => self.left_2(),
            Moves::Front => self.front(),
            Moves::FrontPrime => self.front_prime(),
            Moves::Front2 => self.front_2(),
            Moves::Back => self.back(),
            Moves::BackPrime => self.back_prime(),
            Moves::Back2 => self.back_2(),
        }
    }

    pub fn undo_move(&mut self) {
        if let Some(last_move) = self.moves.pop() {
            self.make_move(last_move.opposite());
        }
    }

    fn rotate_face_clockwise(&mut self, face: usize) {
        let previous = self.pieces;

        self.pieces[face][0][0] = previous[face][2][0];
        self.pieces[face][0][1] = previous[face][1][0];
        self.pieces[face][0][2] = previous[face][0][0];
        self.pieces[face][1][0] = previous[face][2][1];
        // [face][1][1] is the center piece, doesn't move
        self.pieces[face][1][2] = previous[face][0][1];
        self.pieces[face][2][0] = previous[face][2][2];
        self.pieces[face][2][1] = previous[face][1][2];
        self.pieces[face][2][2] = previous[face][0][2];
    }

    fn up(&mut self) {
        let previous = self.pieces;

        self.rotate_face_clockwise(0);

        // Left face
        self.pieces[1][0][0] = previous[2][0][0];
        self.pieces[1][0][1] = previous[2][0][1];
        self.pieces[1][0][2] = previous[2][0][2];

        // Front face
        self.pieces[2][0][0] = previous[3][0][0];
        self.pieces[2][0][1] = previous[3][0][1];
        self.pieces[2][0][2] = previous[3][0][2];

        // Right face
        self.pieces[3][0][0] = previous[4][0][0];
        self.pieces[3][0][1] = previous[4][0][1];
        self.pieces[3][0][2] = previous[4][0][2];

        // Back face
        self.pieces[4][0][0] = previous[1][0][0];
        self.pieces[4][0][1] = previous[1][0][1];
        self.pieces[4][0][2] = previous[1][0][2];
    }
    fn down(&mut self) {
        let previous = self.pieces;

        // Down face
        self.rotate_face_clockwise(5);

        // Left face
        self.pieces[1][2][0] = previous[4][2][0];
        self.pieces[1][2][1] = previous[4][2][1];
        self.pieces[1][2][2] = previous[4][2][2];

        // Front face
        self.pieces[2][2][0] = previous[1][2][0];
        self.pieces[2][2][1] = previous[1][2][1];
        self.pieces[2][2][2] = previous[1][2][2];

        // Right face
        self.pieces[3][2][0] = previous[2][2][0];
        self.pieces[3][2][1] = previous[2][2][1];
        self.pieces[3][2][2] = previous[2][2][2];

        // Back face
        self.pieces[4][2][0] = previous[3][2][0];
        self.pieces[4][2][1] = previous[3][2][1];
        self.pieces[4][2][2] = previous[3][2][2];
    }
    fn right(&mut self) {
        let previous = self.pieces;

        // Right face
        self.rotate_face_clockwise(3);

        // Up face
        self.pieces[0][0][2] = previous[2][0][2];
        self.pieces[0][1][2] = previous[2][1][2];
        self.pieces[0][2][2] = previous[2][2][2];

        // Front face
        self.pieces[2][0][2] = previous[5][0][2];
        self.pieces[2][1][2] = previous[5][1][2];
        self.pieces[2][2][2] = previous[5][2][2];

        // Back face
        self.pieces[4][0][0] = previous[0][2][2];
        self.pieces[4][1][0] = previous[0][1][2];
        self.pieces[4][2][0] = previous[0][0][2];

        // Down face
        self.pieces[5][0][2] = previous[4][2][0];
        self.pieces[5][1][2] = previous[4][1][0];
        self.pieces[5][2][2] = previous[4][0][0];
    }
    fn left(&mut self) {
        let previous = self.pieces;

        // Right face
        self.rotate_face_clockwise(1);

        // Up face
        self.pieces[0][0][0] = previous[4][2][2];
        self.pieces[0][1][0] = previous[4][1][2];
        self.pieces[0][2][0] = previous[4][0][2];

        // Front face
        self.pieces[2][0][0] = previous[0][0][0];
        self.pieces[2][1][0] = previous[0][1][0];
        self.pieces[2][2][0] = previous[0][2][0];

        // Back face
        self.pieces[4][0][2] = previous[5][2][0];
        self.pieces[4][1][2] = previous[5][1][0];
        self.pieces[4][2][2] = previous[5][0][0];

        // Down face
        self.pieces[5][0][0] = previous[2][0][0];
        self.pieces[5][1][0] = previous[2][1][0];
        self.pieces[5][2][0] = previous[2][2][0];
    }
    fn front(&mut self) {
        let previous = self.pieces;

        // Front face
        self.rotate_face_clockwise(2);

        // Up face
        self.pieces[0][2][0] = previous[1][2][2];
        self.pieces[0][2][1] = previous[1][1][2];
        self.pieces[0][2][2] = previous[1][0][2];

        // Left face
        self.pieces[1][0][2] = previous[5][0][0];
        self.pieces[1][1][2] = previous[5][0][1];
        self.pieces[1][2][2] = previous[5][0][2];

        // Right face
        self.pieces[3][0][0] = previous[0][2][0];
        self.pieces[3][1][0] = previous[0][2][1];
        self.pieces[3][2][0] = previous[0][2][2];

        // Down face
        self.pieces[5][0][0] = previous[3][2][0];
        self.pieces[5][0][1] = previous[3][1][0];
        self.pieces[5][0][2] = previous[3][0][0];
    }
    fn back(&mut self) {
        let previous = self.pieces;

        // Back face
        self.rotate_face_clockwise(4);

        // Up face
        self.pieces[0][0][0] = previous[3][0][2];
        self.pieces[0][0][1] = previous[3][1][2];
        self.pieces[0][0][2] = previous[3][2][2];

        // Left face
        self.pieces[1][0][0] = previous[0][0][2];
        self.pieces[1][1][0] = previous[0][0][1];
        self.pieces[1][2][0] = previous[0][0][0];

        // Right face
        self.pieces[3][0][2] = previous[5][2][2];
        self.pieces[3][1][2] = previous[5][2][1];
        self.pieces[3][2][2] = previous[5][2][0];

        // Down face
        self.pieces[5][2][0] = previous[1][0][0];
        self.pieces[5][2][1] = previous[1][1][0];
        self.pieces[5][2][2] = previous[1][2][0];
    }

    fn up_prime(&mut self) {
        self.up();
        self.up();
        self.up();
    }
    fn down_prime(&mut self) {
        self.down();
        self.down();
        self.down();
    }
    fn right_prime(&mut self) {
        self.right();
        self.right();
        self.right();
    }
    fn left_prime(&mut self) {
        self.left();
        self.left();
        self.left();
    }
    fn front_prime(&mut self) {
        self.front();
        self.front();
        self.front();
    }
    fn back_prime(&mut self) {
        self.back();
        self.back();
        self.back();
    }

    fn up_2(&mut self) {
        self.up();
        self.up();
    }
    fn down_2(&mut self) {
        self.down();
        self.down();
    }
    fn right_2(&mut self) {
        self.right();
        self.right();
    }
    fn left_2(&mut self) {
        self.left();
        self.left();
    }
    fn front_2(&mut self) {
        self.front();
        self.front();
    }
    fn back_2(&mut self) {
        self.back();
        self.back();
    }
}
