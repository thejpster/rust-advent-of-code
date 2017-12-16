use std::collections::VecDeque;

enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

// struct Buffer {
//     index: HashMap<char, usize>,
// }

// impl Buffer {
//     fn new(len: usize) -> Buffer {
//         Buffer {
//             index: HashMap::new()
//         }
//     }
// }

pub fn run(contents: &Vec<Vec<String>>) {
    let mut programs: VecDeque<char> = (0..16).map(|x| (x + 97u8).into()).collect();
    let steps = contents[0][0].split(",");
    let steps: Vec<Move> = steps.map(|x| decode(x)).collect();
    println!("Programs: {:?}", programs);

    for i in 0..1_000_000_000 {
        for step in steps.iter() {
            match step {
                &Move::Exchange(a, b) => exchange(&mut programs, a, b),
                &Move::Spin(s) => spin(&mut programs, s),
                &Move::Partner(a, b) => partner(&mut programs, a, b),
            }
        }
        if i % 1_000_000 == 0 {
            println!("Programs ({}): {}", i, programs.iter().collect::<String>());
        }
    }
    println!("Programs: {}", programs.iter().collect::<String>());
}

fn decode(input: &str) -> Move {
    match &input[0..1] {
        "s" => Move::Spin(input[1..].parse().unwrap()),
        "x" => {
            let mut parts = input[1..].split("/").map(|x| x.parse::<usize>().unwrap());
            Move::Exchange(parts.next().unwrap(), parts.next().unwrap())
        }
        "p" => {
            let mut parts = input[1..].split("/").map(|x| x.chars().nth(0).unwrap());
            Move::Partner(parts.next().unwrap(), parts.next().unwrap())
        }
        _ => panic!("Bad command"),
    }
}

fn exchange(programs: &mut VecDeque<char>, x: usize, y: usize) {
    programs.swap(x, y);
}

fn spin(programs: &mut VecDeque<char>, shift: usize) {
    // spin(abcde, 3) = cdeab
    let offset = programs.len() - shift;
    let tail = programs.split_off(offset);
    let head = programs.split_off(0);
    programs.extend(tail);
    programs.extend(head);
}

fn partner(programs: &mut VecDeque<char>, c1: char, c2: char) {
    let mut i1 = None;
    let mut i2 = None;
    for i in 0..programs.len() {
        if programs[i] == c1 {
            i1 = Some(i);
        }
        if programs[i] == c2 {
            i2 = Some(i);
        }
    };
    programs.swap(i1.unwrap(), i2.unwrap());
}
