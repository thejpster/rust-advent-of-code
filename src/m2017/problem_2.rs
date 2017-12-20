pub fn run(contents: &[Vec<String>]) {
    let spreadsheet = &contents[0];
    let mut cs = 0;
    let mut cs2 = 0;
    for line in spreadsheet {
        let cells: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        for cell1 in &cells {
            for cell2 in &cells {
                if (cell1 != cell2) && (*cell1 % *cell2 == 0) {
                    cs2 += *cell1 / *cell2;
                }
            }
        }
        let max = cells.iter().cloned().max().unwrap();
        let min = cells.iter().cloned().min().unwrap();
        cs += max - min;
    }
    println!("CS {}", cs);
    println!("CS2 {}", cs2);
}
