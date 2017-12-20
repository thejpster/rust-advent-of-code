#[derive(Debug)]
enum Pins {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

#[derive(Debug)]
enum Pins2 {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    A,
    B,
    C,
    D,
}

impl ::std::ops::Add<char> for Pins {
    type Output = Pins;

    fn add(self, rhs: char) -> Pins {
        match rhs {
            'U' => match self {
                Pins::One | Pins::Four => Pins::One,
                Pins::Two | Pins::Five => Pins::Two,
                Pins::Three | Pins::Six => Pins::Three,
                Pins::Seven => Pins::Four,
                Pins::Eight => Pins::Five,
                Pins::Nine => Pins::Six,
            },
            'D' => match self {
                Pins::One => Pins::Four,
                Pins::Two => Pins::Five,
                Pins::Three => Pins::Six,
                Pins::Four | Pins::Seven => Pins::Seven,
                Pins::Five | Pins::Eight => Pins::Eight,
                Pins::Six | Pins::Nine => Pins::Nine,
            },
            'L' => match self {
                Pins::One | Pins::Two => Pins::One,
                Pins::Three => Pins::Two,
                Pins::Four | Pins::Five => Pins::Four,
                Pins::Six => Pins::Five,
                Pins::Seven | Pins::Eight => Pins::Seven,
                Pins::Nine => Pins::Eight,
            },
            'R' => match self {
                Pins::One => Pins::Two,
                Pins::Two | Pins::Three => Pins::Three,
                Pins::Four => Pins::Five,
                Pins::Five | Pins::Six => Pins::Six,
                Pins::Seven => Pins::Eight,
                Pins::Eight | Pins::Nine => Pins::Nine,
            },
            _ => panic!("Unsupported movement"),
        }
    }
}

//     1
//   2 3 4
// 5 6 7 8 9
//   A B C
//     D
impl ::std::ops::Add<char> for Pins2 {
    type Output = Pins2;

    fn add(self, rhs: char) -> Pins2 {
        match rhs {
            'U' => match self {
                Pins2::One | Pins2::Three => Pins2::One,
                Pins2::Two | Pins2::Six => Pins2::Two,
                Pins2::Seven => Pins2::Three,
                Pins2::Eight | Pins2::Four => Pins2::Four,
                Pins2::Five => Pins2::Five,
                Pins2::A => Pins2::Six,
                Pins2::B => Pins2::Seven,
                Pins2::C => Pins2::Eight,
                Pins2::Nine => Pins2::Nine,
                Pins2::D => Pins2::B,
            },
            'D' => match self {
                Pins2::One => Pins2::Three,
                Pins2::Five => Pins2::Five,
                Pins2::Two => Pins2::Six,
                Pins2::Three => Pins2::Seven,
                Pins2::Four => Pins2::Eight,
                Pins2::Nine => Pins2::Nine,
                Pins2::Six | Pins2::A => Pins2::A,
                Pins2::Seven => Pins2::B,
                Pins2::C | Pins2::Eight => Pins2::C,
                Pins2::B | Pins2::D => Pins2::D,
            },
            'L' => match self {
                Pins2::One => Pins2::One,
                Pins2::Two | Pins2::Three => Pins2::Two,
                Pins2::Four => Pins2::Three,
                Pins2::Five | Pins2::Six => Pins2::Five,
                Pins2::Seven => Pins2::Six,
                Pins2::Eight => Pins2::Seven,
                Pins2::Nine => Pins2::Eight,
                Pins2::A | Pins2::B => Pins2::A,
                Pins2::C => Pins2::B,
                Pins2::D => Pins2::D,
            },
            'R' => match self {
                Pins2::One => Pins2::One,
                Pins2::Two => Pins2::Three,
                Pins2::Three | Pins2::Four => Pins2::Four,
                Pins2::Five => Pins2::Six,
                Pins2::Six => Pins2::Seven,
                Pins2::Seven => Pins2::Eight,
                Pins2::Eight | Pins2::Nine => Pins2::Nine,
                Pins2::A => Pins2::B,
                Pins2::B | Pins2::C => Pins2::C,
                Pins2::D => Pins2::D,
            },
            _ => panic!("Unsupported movement"),
        }
    }
}

pub fn run(contents: &[Vec<String>]) {
    let mut digit = Pins::Five;
    for line in &contents[0] {
        for step in line.chars() {
            digit = digit + step;
            // println!("{:?} => {:?}", step, digit);
        }
        println!("{:?}", digit);
    }

    println!("***");

    let mut digit = Pins2::Five;
    for line in &contents[0] {
        for step in line.chars() {
            digit = digit + step;
            // println!("{:?} => {:?}", step, digit);
        }
        println!("{:?}", digit);
    }
}
