use std::collections::HashSet;

pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    let mut result = 0;
    let mut has_item = vec![false; 53];
    for line in input.lines() {
        for v in &mut has_item {
            *v = false;
        }
        let second_half_start = line.len() / 2;
        for (i, char) in line.chars().enumerate() {
            let prio = get_priority(char)?;
            let found = &mut has_item[prio as usize];
            if i < second_half_start {
                *found = true;
            }
            else if i >= second_half_start && *found {
                result += prio;
                break;
            }
        }
    }
    Ok(result.to_string())
}

pub fn process_part_two(input: &str) -> Result<String, &'static str> {
    let mut result = 0;
    let mut item_count = vec![0; 53];
    for (i, line) in input.lines().enumerate() {
        if i % 3 == 0 {
            for v in &mut item_count {
                *v = 0;
            }
        }
        for ch in HashSet::<char>::from_iter(line.chars()) {
            let prio = get_priority(ch)?;
            let count = &mut item_count[prio as usize];
            *count += 1;
            if *count == 3 {
                result += prio;
                break;
            }
        }
    }
    Ok(result.to_string())
}

fn get_priority(ch: char) -> Result<u32, &'static str> {
    if !ch.is_ascii_alphabetic() {
        return Err("invalid character encountered");
    }
    let base = if ch.is_uppercase() { 'A' as u32 } else { 'a' as u32 };
    let offset = if ch.is_uppercase() { 27 } else { 1 };
    Ok(ch as u32 - base + offset)
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

    #[test]
    fn test_part_two() {
        assert_eq!(process_part_two(INPUT).unwrap(), "70");
    }
}
