pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    Ok(input.to_string())
}

pub fn process_part_two(input: &str) -> Result<String, &'static str> {
    Ok(input.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(INPUT).unwrap(), "2");
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(process_part_two(INPUT).unwrap(), "<solution>");
    }
}
