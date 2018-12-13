use std::collections::HashSet;

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let numbers = crate::parse_numbers(&contents[0])?;
    let sum: i64 = numbers.iter().cloned().sum();
    println!("First sum: {:?}", sum);
    let mut map = HashSet::new();
    let mut sum = 0;
    for freq in numbers.iter().cycle() {
        sum += freq;
        if map.contains(&sum) {
            println!("Duplicate: {}", sum);
            break;
        } else {
            map.insert(sum);
        }
    }
    Ok(())
}
