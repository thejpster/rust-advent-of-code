use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    // Big one line string
    let number_line = &contents[0][0];
    let numbers: Vec<u32> = number_line
        .chars()
        .map(|x| {
            // println!("Got {}", x);
            x.to_digit(10).ok_or_else(|| format_err!("Bad digit {}", x))
        })
        .collect::<Result<_, _>>()?;
    for number in &numbers {
        println!("Number: {}", number);
    }
    Ok(())
}
