use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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

impl Moves {
    pub fn from_notation(notation: &str) -> Option<Self> {
        match notation {
            "U" => Some(Moves::Up),
            "U'" => Some(Moves::UpPrime),
            "U2" => Some(Moves::Up),

            "D" => Some(Moves::Down),
            "D'" => Some(Moves::DownPrime),
            "D2" => Some(Moves::Down2),

            "R" => Some(Moves::Right),
            "R'" => Some(Moves::RightPrime),
            "R2" => Some(Moves::Right2),

            "L" => Some(Moves::Left),
            "L'" => Some(Moves::LeftPrime),
            "L2" => Some(Moves::Left2),

            "F" => Some(Moves::Front),
            "F'" => Some(Moves::FrontPrime),
            "F2" => Some(Moves::Front2),

            "B" => Some(Moves::Back),
            "B'" => Some(Moves::BackPrime),
            "B2" => Some(Moves::Back2),

            _ => None,
        }
    }

    pub fn opposite(&self) -> Moves {
        match self {
            Moves::Up => Moves::UpPrime,
            Moves::UpPrime => Moves::Up,
            Moves::Up2 => Moves::Up2,
            Moves::Down => Moves::DownPrime,
            Moves::DownPrime => Moves::Down,
            Moves::Down2 => Moves::Down2,
            Moves::Right => Moves::RightPrime,
            Moves::RightPrime => Moves::Right,
            Moves::Right2 => Moves::Right2,
            Moves::Left => Moves::LeftPrime,
            Moves::LeftPrime => Moves::Left,
            Moves::Left2 => Moves::Left2,
            Moves::Front => Moves::FrontPrime,
            Moves::FrontPrime => Moves::Front,
            Moves::Front2 => Moves::Front2,
            Moves::Back => Moves::BackPrime,
            Moves::BackPrime => Moves::Back,
            Moves::Back2 => Moves::Back2,
        }
    }
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
