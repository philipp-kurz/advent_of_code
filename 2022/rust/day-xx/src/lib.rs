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
";

    #[test]
    fn test_part_one() {
        assert_eq!(process_part_one(INPUT).unwrap(), "<solution>");
    }

    // #[test]
    // fn test_part_two() {
    //     assert_eq!(process_part_two(INPUT).unwrap(), "<solution>");
    // }
}
