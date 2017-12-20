use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut count = 0;
    for line in &contents[0] {
        let parts: Vec<u32> = line.split_whitespace()
            .map(|x| x.parse())
            .collect::<Result<_, _>>()?;
        if possible(parts) {
            count += 1;
        }
    }
    println!("Count: {}", count);

    count = 0;
    for line in contents[0].chunks(3) {
        let parts: Vec<Vec<u32>> = line.iter()
            .map(|x| x.split_whitespace().map(|x| x.parse()).collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;
        let tri1 = vec![parts[0][0], parts[1][0], parts[2][0]];
        let tri2 = vec![parts[0][1], parts[1][1], parts[2][1]];
        let tri3 = vec![parts[0][2], parts[1][2], parts[2][2]];
        if possible(tri1) {
            count += 1;
        }
        if possible(tri2) {
            count += 1;
        }
        if possible(tri3) {
            count += 1;
        }
    }

    println!("Count: {}", count);
    Ok(())
}

fn possible(mut v: Vec<u32>) -> bool {
    v.sort();
    v[0] + v[1] > v[2]
}
