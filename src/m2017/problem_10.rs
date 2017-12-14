const MAX: u8 = 255;

pub fn run(contents: &Vec<Vec<String>>) {
    let mut items: Vec<u8> = (0..MAX).collect();
    // Don't have inclusive range syntax, so manually push on the last item
    items.push(MAX);

    part1(&mut items.clone(), &contents[0][0]);
    part2(&mut items, &contents[0][0]);
}

fn part1(items: &mut [u8], contents: &str) {
    let lengths: Vec<u8> = contents.split(",").map(|x| x.parse().unwrap()).collect();
    round(items, &lengths, 1);
    println!("{}", items[0] as u16 * items[1] as u16);
}

fn part2(items: &mut [u8], contents: &str) {
    let hash = calculate(items, contents);
    let dense_hash: String = hash.iter().map(|x| format!("{:02x}", x)).collect();
    println!("Dense Hash: {}", dense_hash);
}

pub fn calculate(items: &mut [u8], contents: &str) -> Vec<u8> {
    let mut lengths: Vec<u8> = contents.bytes().collect();
    lengths.extend(&[17, 31, 73, 47, 23]);
    round(items, &lengths, 64);
    items[..]
        .chunks(16)
        .map(|c| c.iter().fold(0, |acc, x| acc ^ x))
        .collect::<Vec<u8>>()
}

fn round(items: &mut [u8], lengths: &[u8], count: usize) {
    let mut skip: usize = 0;
    let mut start: usize = 0;
    for _ in 0..count {
        for x in lengths {
            let length = *x as usize;
            reverse(items, start, length);
            start = (start + length + skip) % items.len();
            skip = skip + 1;
        }
    }
}

fn reverse(items: &mut [u8], mut start: usize, mut length: usize) {
    while length >= 2 {
        let end = (start + length - 1) % items.len();
        items.swap(start, end);
        start = (start + 1) % items.len();
        length = length - 2;
    }
}
