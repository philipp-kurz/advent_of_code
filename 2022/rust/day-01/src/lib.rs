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
        Part::One => process_part_one(&input)?,
        Part::Two => process_part_two(&input)?,
    };
    println!("Calories: {calories}");
    Ok(())
}

pub fn process_part_one(input: &str) -> Result<i32, ParseIntError> {
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

pub fn process_part_one_2(input: &str) -> Result<i32, ParseIntError> {
    let calories = input
        .trim_end()
        .split("\n\n")
        .map(|elf_lines| {
            elf_lines
                .lines()
                .map(|str_val| { str_val.parse::<i32>().unwrap() })
                .sum()
        }).max()
        .unwrap();
    Ok(calories)
}

pub fn process_part_two(input: &str) -> Result<i32, ParseIntError> {
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

pub fn process_part_two_2(input: &str) -> Result<i32, ParseIntError> {
    let mut result = input
        .trim_end()
        .split("\n\n")
        .map(|elf_lines| {
            elf_lines
                .lines()
                .map(|str_val| { str_val.parse::<i32>().unwrap() })
                .sum()
        })
        .collect::<Vec<_>>();
    result.sort();
    let calories = result
        .iter()
        .rev()
        .take(3)
        .sum();
    Ok(calories)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(sample_input()).unwrap(), 24000);
    }

    #[test]
    fn test_part_one_v2() {
        assert_eq!(process_part_one_2(sample_input()).unwrap(), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(sample_input()).unwrap(), 45000);
    }

    #[test]
    fn test_part_two_v2() {
        assert_eq!(process_part_two_2(sample_input()).unwrap(), 45000);
    }

    fn sample_input() -> &'static str { "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

}

