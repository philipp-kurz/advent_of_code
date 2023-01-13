enum Outcome {
    Win,
    Tie,
    Lose,
}

impl Outcome {
    fn get_points(&self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Tie => 3,
            Self::Lose => 0,
        }
    }

    fn from_encoding(enc: char) -> Option<Self> {
        match enc {
            'X' => Some(Self::Lose),
            'Y' => Some(Self::Tie),
            'Z' => Some(Self::Win),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_opponent(encoded_move: char) -> Option<Self> {
        match encoded_move {
            'A' => Some(Self::Rock),
            'B' => Some(Self::Paper),
            'C' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn from_own(encoded_move: char) -> Option<Self> {
        match encoded_move {
            'X' => Some(Self::Rock),
            'Y' => Some(Self::Paper),
            'Z' => Some(Self::Scissors),
            _ => None,
        }
    }

    fn defeats(&self, opponent_move: Self) -> Outcome {
        if *self == opponent_move {
            return Outcome::Tie;
        } else if (*self == Self::Rock && opponent_move == Self::Scissors) ||
                  (*self == Self::Paper && opponent_move == Self::Rock) ||
                  (*self == Self::Scissors && opponent_move == Self::Paper) {
            return Outcome::Win;
        } else {
            Outcome::Lose
        }
    }

    fn get_points(&self) -> u32{
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn get_required_own_move(&self, outcome: &Outcome) -> Move {
        match self {
            Self::Rock => match outcome {
                Outcome::Lose => Self::Scissors,
                Outcome::Tie => Self::Rock,
                Outcome::Win => Self::Paper,
            },
            Self::Paper => match outcome {
                Outcome::Lose => Self::Rock,
                Outcome::Tie => Self::Paper,
                Outcome::Win => Self::Scissors,
            }
            Self::Scissors => match outcome {
                Outcome::Lose => Self::Paper,
                Outcome::Tie => Self::Scissors,
                Outcome::Win => Self::Rock,
            }
        }
    }
}

fn get_encodings_from_line(line: &str) -> Result<Vec<char>, &'static str> {
    let encodings: Vec<char> = line
        .split(" ")
        .flat_map(|sub| sub.chars())
        .collect();
    if encodings.len() != 2 {
        return Err("Invalid match line encountered");
    }
    Ok(encodings)
}

const INV_MOVE_ERR: &str = "Invalid encoded move encountered";

fn get_match_points_p1(line: &str) -> Result<u32, &'static str> {
    let enc_moves = get_encodings_from_line(line)?;
    let opponent_move = Move::from_opponent(enc_moves[0]).ok_or(INV_MOVE_ERR)?;
    let own_move = Move::from_own(enc_moves[1]).ok_or(INV_MOVE_ERR)?;
    let outcome = own_move.defeats(opponent_move);
    Ok(own_move.get_points() + outcome.get_points())
}

fn get_match_points_p2(line: &str) -> Result<u32, &'static str> {
    let encodings = get_encodings_from_line(line)?;
    let opponent_move = Move::from_opponent(encodings[0]).ok_or(INV_MOVE_ERR)?;
    let outcome = Outcome::from_encoding(encodings[1]).ok_or(INV_MOVE_ERR)?;
    let own_move = opponent_move.get_required_own_move(&outcome);
    Ok(own_move.get_points() + outcome.get_points())
}

pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    let mut result = 0;
    for match_points in input.trim_end().lines().map(get_match_points_p1) {
        result += match_points?;
    }
    Ok(result.to_string())
}

pub fn process_part_two(input: &str) -> Result<String, &'static str> {
    let mut result = 0;
    for match_points in input.trim_end().lines().map(get_match_points_p2) {
        result += match_points?;
    }
    Ok(result.to_string())
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
A Y
B X
C Z";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(INPUT).unwrap(), "15");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(INPUT).unwrap(), "12");
    }
}
