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
                  (*self == Self::Scissors && opponent_move == Move::Paper) {
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

}

fn get_match_points(line: &str) -> Result<u32, &'static str>{
    let enc_moves: Vec<char> = line
        .split(" ")
        .flat_map(|sub| sub.chars())
        .collect();
    if enc_moves.len() != 2 {
        return Err("Invalid match line encountered");
    }
    let inv_move_err = "Invalid encoded move encountered";
    let opponent_move = Move::from_opponent(enc_moves[0]).ok_or(inv_move_err)?;
    let own_move = Move::from_own(enc_moves[1]).ok_or(inv_move_err)?;
    let outcome = own_move.defeats(opponent_move);
    Ok(own_move.get_points() + outcome.get_points())
}

pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    let mut result = 0;
    for match_points in input.trim_end().lines().map(get_match_points) {
        result += match_points?;
    }
    Ok(result.to_string())
}

pub fn process_part_two(_input: &str) -> String {
    "".to_string()
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
}
