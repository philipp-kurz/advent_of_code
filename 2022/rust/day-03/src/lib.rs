pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    let result = 0;
    let mut has_item = vec![false; 52];
    for line in input.lines() {
        for v in &mut has_item {
            *v = false;
        }
        let second_half_start = line.len() / 2;
        for char in line.chars() {

        }
    }
    Ok(input.to_string())
}

pub fn process_part_two(input: &str) -> Result<String, &'static str> {
    Ok(input.to_string())
}

fn get_priority(ch: char) -> Result<u32, &'static str> {
    static A_ASCII: u32 = 'a'.to_digit(10)
        .expect("Constant letter will always be converted successfully");
    match ch.to_digit(10) {
        Some(val) => Ok(val - A_ASCII + 1),
        None => Err("failed to convert character")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(INPUT).unwrap(), "157");
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(process_part_two(INPUT).unwrap(), "<solution>");
    // }
}
