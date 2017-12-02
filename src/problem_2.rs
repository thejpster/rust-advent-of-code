pub fn run(contents: &Vec<Vec<String>>) {
    let spreadsheet = &contents[0];
    let mut cs = 0;
    let mut cs2 = 0;
    for line in spreadsheet {
        let mut min = 1 << 31;
        let mut max = 0;
        let cells: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for cell1 in &cells {
            if *cell1 > max {
                max = *cell1;
            }
            if *cell1 < min {
                min = *cell1;
            }
            for cell2 in &cells {
                if cell1 != cell2 {
                    if *cell1 % *cell2 == 0 {
                        cs2 = cs2 + (*cell1 / *cell2);
                    }
                }
            }
        }
        cs = cs + (max - min);
    }
    println!("CS {}", cs);
    println!("CS2 {}", cs2);
}
