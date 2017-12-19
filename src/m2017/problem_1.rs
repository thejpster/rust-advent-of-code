pub fn run(contents: &[Vec<String>]) {
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
    calc(&numbers, 1);
    calc(&numbers, numbers.len() / 2);
}

fn calc(numbers: &[u32], offset: usize) {
    println!(
        "{}",
        numbers
            .iter()
            .zip(numbers.iter().skip(offset).chain(numbers.iter()))
            .filter(|x| x.0 == x.1)
            .map(|x| *x.0)
            .sum::<u32>()
    );
}
