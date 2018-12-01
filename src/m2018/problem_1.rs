use std::collections::HashSet;

use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    // Big one line string
    let number_line = &contents[0];
    let numbers: Vec<i32> = number_line.iter().map(|x| x.parse()).collect::<Result<_, _>>()?;
    let sum = numbers.iter().cloned().sum::<i32>();
    println!("First sum {:?}", sum);
    let mut map = HashSet::new();
    let mut sum = 0;
    for freq in numbers.iter().cycle() {
        sum = sum + freq;
        if map.contains(&sum) {
            println!("Duplicate: {}", sum);
            break;
        } else {
            map.insert(sum);
        }
    }
    Ok(())
}
