use std::fmt;

#[derive(Clone, Copy)]
pub enum Moves {
    Up,
    UpPrime,
    Up2,
    Down,
    DownPrime,
    Down2,
    Right,
    RightPrime,
    Right2,
    Left,
    LeftPrime,
    Left2,
    Front,
    FrontPrime,
    Front2,
    Back,
    BackPrime,
    Back2,
}

impl fmt::Display for Moves {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let notation = match self {
            Moves::Up => "U",
            Moves::UpPrime => "U'",
            Moves::Up2 => "U2",
            Moves::Down => "D",
            Moves::DownPrime => "D'",
            Moves::Down2 => "D2",
            Moves::Right => "R",
            Moves::RightPrime => "R'",
            Moves::Right2 => "R2",
            Moves::Left => "L",
            Moves::LeftPrime => "L'",
            Moves::Left2 => "L2",
            Moves::Front => "F",
            Moves::FrontPrime => "F'",
            Moves::Front2 => "F2",
            Moves::Back => "B",
            Moves::BackPrime => "B'",
            Moves::Back2 => "B2",
        };

        write!(f, "{}", notation)
    }
}
