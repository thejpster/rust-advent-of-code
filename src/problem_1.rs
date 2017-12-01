pub fn run(contents: &Vec<Vec<String>>) {
    // Big one line string
    let number_line = &contents[0][0];
    let numbers: Vec<u32> = number_line
        .chars()
        .map(|x| {
            // println!("Got {}", x);
            x.to_digit(10).unwrap()
        })
        .collect();
    // for number in &numbers {
    //     println!("Number: {}", number);
    // }
    let offset = numbers.len() / 2;
    let mut sum = 0;
    for idx in 0..numbers.len() {
        let x = numbers[idx];
        let y = numbers[(idx + offset) % numbers.len()];
        if x == y {
            sum = sum + x;
        }
    }
    println!("Sum: {}", sum);
}
