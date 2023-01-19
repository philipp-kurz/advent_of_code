use std::error::Error;
use std::fs;

// Correct answer for input.txt: 196804

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let result = day_01::process_part_two(&input)?;
    println!("Result: {result}");
    Ok(())
}
