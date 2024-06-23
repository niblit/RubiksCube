use std::fmt;

use colored::{ColoredString, Colorize};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Orange,
    Green,
    Red,
    Blue,
    Yellow,
}

pub const INITIAL_POSITION: [[[Color; 3]; 3]; 6] = [
    [[Color::White; 3]; 3],
    [[Color::Orange; 3]; 3],
    [[Color::Green; 3]; 3],
    [[Color::Red; 3]; 3],
    [[Color::Blue; 3]; 3],
    [[Color::Yellow; 3]; 3],
];

pub struct Cube {
    pieces: [[[Color; 3]; 3]; 6],
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn piece_to_square(cube: &Cube, face: usize, row: usize, col: usize) -> ColoredString {
            let color = cube.pieces[face][row][col];
            match color {
                Color::White => if row == 1 && col == 1 {" ▪ "} else {" W "}.on_truecolor(255, 255, 255),
                Color::Orange => if row == 1 && col == 1 {" ▪ "} else {" O "}.on_truecolor(255, 165, 0),
                Color::Green => if row == 1 && col == 1 {" ▪ "} else {" G "}.on_truecolor(0, 255, 0),
                Color::Red => if row == 1 && col == 1 {" ▪ "} else {" R "}.on_truecolor(255, 0, 0),
                Color::Blue => if row == 1 && col == 1 {" ▪ "} else {" B "}.on_truecolor(0, 0, 255),
                Color::Yellow => if row == 1 && col == 1 {" ▪ "} else {" Y "}.on_truecolor(255, 255, 0),
            }
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
        }
    }
}

impl Cube {
    pub fn scramble(&mut self, scramble: &str) {
        let moves: Vec<&str> = scramble.split(' ').collect();

        for r#move in moves {
            match r#move {
                "U" => self.up(),
                "U'" => self.up_prime(),
                "U2" => self.up_2(),

                "D" => self.down(),
                "D'" => self.down_prime(),
                "D2" => self.down_2(),

                "R" => self.right(),
                "R'" => self.right_prime(),
                "R2" => self.right_2(),

                "L" => self.left(),
                "L'" => self.left_prime(),
                "L2" => self.left_2(),

                "F" => self.front(),
                "F'" => self.front_prime(),
                "F2" => self.front_2(),

                "B" => self.back(),
                "B'" => self.back_prime(),
                "B2" => self.back_2(),

                _ => {panic!("Invalid move");}
            }
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

    pub fn up(&mut self) {
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
    pub fn down(&mut self) {
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
    pub fn right(&mut self) {
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
    pub fn left(&mut self) {
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
    pub fn front(&mut self) {
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
    pub fn back(&mut self) {
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

    pub fn up_prime(&mut self) {
        self.up();
        self.up();
        self.up();
    }
    pub fn down_prime(&mut self) {
        self.down();
        self.down();
        self.down();
    }
    pub fn right_prime(&mut self) {
        self.right();
        self.right();
        self.right();
    }
    pub fn left_prime(&mut self) {
        self.left();
        self.left();
        self.left();
    }
    pub fn front_prime(&mut self) {
        self.front();
        self.front();
        self.front();
    }
    pub fn back_prime(&mut self) {
        self.back();
        self.back();
        self.back();
    }

    pub fn up_2(&mut self) {
        self.up();
        self.up();
    }
    pub fn down_2(&mut self) {
        self.down();
        self.down();
    }
    pub fn right_2(&mut self) {
        self.right();
        self.right();
    }
    pub fn left_2(&mut self) {
        self.left();
        self.left();
    }
    pub fn front_2(&mut self) {
        self.front();
        self.front();
    }
    pub fn back_2(&mut self) {
        self.back();
        self.back();
    }
}
