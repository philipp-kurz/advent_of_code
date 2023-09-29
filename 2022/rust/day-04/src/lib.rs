fn fully_contained(line: &str) -> bool {
    let bounds = line
        .split(&[',', '-'])
        .map(|bound| {
            bound.parse::<u32>().unwrap()
        })
        .collect::<Vec<_>>();
    let [start_1, end_1, start_2, end_2] = <[_; 4]>::try_from(&bounds[0..4]).unwrap();
    (start_1 >= start_2 && end_1 <= end_2) || (start_2 >= start_1 && end_2 <= end_1)
}

fn between(value: u32, start: u32, end: u32) -> bool {
    value >= start && value <= end
}

fn does_overlap(line: &str) -> bool {
    let bounds = line
        .split(&[',', '-'])
        .map(|bound| {
            bound.parse::<u32>().unwrap()
        })
        .collect::<Vec<_>>();
    let [start_1, end_1, start_2, end_2] = <[_; 4]>::try_from(&bounds[0..4]).unwrap();

    between(start_1, start_2, end_2) ||
    between(end_1, start_2, end_2) ||
    between(start_2, start_1, end_1) ||
    between(end_2, start_1, end_1)
}

pub fn process_part_one(input: &str) -> Result<String, &'static str> {
    let result = input
        .lines()
        .map(|line| fully_contained(line))
        .map(|res| {res as u32})
        .sum::<u32>();
    Ok(result.to_string())
}

pub fn process_part_two(input: &str) -> Result<String, &'static str> {
    let result = input
        .lines()
        .map(|line| does_overlap(line))
        .map(|res| {res as u32})
        .sum::<u32>();
    Ok(result.to_string())
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
    fn test_part_two() {
        assert_eq!(process_part_two(INPUT).unwrap(), "4");
    }
}
