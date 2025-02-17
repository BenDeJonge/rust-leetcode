use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Easy = 1,
    Medium = 2,
    Hard = 3,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct InvalidDifficulty {}

impl TryFrom<usize> for Difficulty {
    type Error = InvalidDifficulty;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            x if x == Difficulty::Easy as usize => Ok(Difficulty::Easy),
            x if x == Difficulty::Medium as usize => Ok(Difficulty::Medium),
            x if x == Difficulty::Hard as usize => Ok(Difficulty::Hard),
            _ => Err(InvalidDifficulty {}),
        }
    }
}

impl Display for InvalidDifficulty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidDifficulty")
    }
}

impl FromStr for Difficulty {
    type Err = InvalidDifficulty;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "easy" | "Easy" | "EASY" => Ok(Self::Easy),
            "medium" | "Medium" | "MEDIUM" => Ok(Self::Medium),
            "hard" | "Hard" | "HARD" => Ok(Self::Hard),
            _ => Err(InvalidDifficulty {}),
        }
    }
}
