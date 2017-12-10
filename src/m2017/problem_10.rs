const MAX: u8 = 255;

fn run1(contents: &Vec<Vec<String>>) {
    let mut items: Vec<u8> = (0..MAX).collect();
    // Don't have inclusive range syntax, so manually push on the last item
    items.push(MAX);

    let lengths: Vec<u8> = contents[0][0]
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();
    // println!("Lengths: {:?}", items);

    round(&mut items, &lengths, 1);

    println!(
        "{} x {} = {}",
        items[0],
        items[1],
        (items[0] as u32) * (items[1] as u32)
    );
}

fn round(items: &mut Vec<u8>, lengths: &Vec<u8>, count: usize) {
    let mut skip: usize = 0;
    let mut start: usize = 0;

    for _ in 0..count {
        for x in lengths {
            let length = *x as usize;
            // println!("****");
            // println!("Items: {:?}", items);
            // println!("Reversing {} from {} to {}", length, start, end);
            reverse(items, start, length);
            // println!("Items: {:?}", items);
            start = (start + length + skip) % items.len();
            // println!("Start: {}", start);
            skip = skip + 1;
        }
    }
}

fn run2(contents: &Vec<Vec<String>>) {
    let mut items: Vec<u8> = (0..MAX).collect();
    // Don't have inclusive range syntax, so manually push on the last item
    items.push(MAX);

    let mut lengths: Vec<u8> = contents[0][0].bytes().collect();
    lengths.extend(&[17, 31, 73, 47, 23]);
    round(&mut items, &lengths, 64);
    let dense_hash: Vec<u8> = items[..].chunks(16).map(|c| fold(c)).collect();
    let dh_hex: String = dense_hash.iter().map(|x| format!("{:02x}", x)).collect();
    println!("Dense Hash: {}", dh_hex);
}

fn fold(items: &[u8]) -> u8 {
    let mut result = 0u8;
    for i in items {
        result = result ^ i;
    }
    result
}

pub fn run(contents: &Vec<Vec<String>>) {
    run1(contents);
    run2(contents);
}

fn reverse(vec: &mut Vec<u8>, mut start: usize, mut length: usize) {
    while length >= 2 {
        let end = (start + length - 1) % vec.len();
        // println!("Swapping #{} {} with #{} {}", start, vec[start], end, vec[end]);
        let temp = vec[end];
        vec[end] = vec[start];
        vec[start] = temp;
        start = (start + 1) % vec.len();
        length = length - 2;
    }
}
