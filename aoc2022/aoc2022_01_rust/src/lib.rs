use std::error;
use std::fs;
use std::cmp;
use std::num::ParseIntError;

enum Part {
    One,
    Two
}

pub struct Config {
    input_path: String,
    part: Part,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3 {
            return Err("invalid number of arguments");
        }
        let input_path = args[1].clone();
        let part = match args[2].as_str() {
            "one" => Part::One,
            "two" => Part::Two,
            _ => return Err("invalid part specified"),
        };
        Ok( Config { input_path, part } )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let input = fs::read_to_string(config.input_path)?;
    let calories = match config.part {
        Part::One => get_max_calories(input)?,
        Part::Two => get_three_max_calories(input)?,
    };
    println!("Calories: {calories}");
    Ok(())
}

fn get_max_calories(input: String) -> Result<i32, ParseIntError> {
    let mut curr_calories = 0;
    let mut max_calories = 0;

    for line in input.lines() {
        if line.len() == 0 {
            max_calories = cmp::max(curr_calories, max_calories);
            curr_calories = 0;
        } else {
            curr_calories += line.to_string().parse::<i32>()?;
        }
    }
    max_calories = cmp::max(curr_calories, max_calories);

    Ok(max_calories)
}

fn get_three_max_calories(input: String) -> Result<i32, ParseIntError> {
    let mut top_three_calories = vec![0; 3];
    let mut curr_calories = 0;

    for line in input.lines() {
        if line.len() == 0 {
            if curr_calories > top_three_calories[0] {
                top_three_calories[0] = curr_calories;
                top_three_calories.sort();
            }
            curr_calories = 0;
        } else {
            curr_calories += line.to_string().parse::<i32>()?;
        }
    }
    top_three_calories[0] = cmp::max(top_three_calories[0], curr_calories);

    Ok(top_three_calories.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(get_max_calories(sample_input()).unwrap(), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(get_three_max_calories(sample_input()).unwrap(), 45000);
    }

    fn sample_input() -> String { "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string()
    }

}

