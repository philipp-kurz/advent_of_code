use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input.txt")?;
    let result = day_02::process_part_one(&input)?;
    println!("Result: {result}");
    Ok(())
}
