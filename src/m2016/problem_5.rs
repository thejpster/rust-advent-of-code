use md5;

const DOOR_ID: &'static str = "wtnhxymk";
// const DOOR_ID:&'static str = "abc";

pub fn run(_contents: &[Vec<String>]) {
    let mut count = 0;
    for i in 1..99999999 {
        // println!("i = {}", i);
        let input = format!("{}{}", DOOR_ID, i);
        // println!("input = {}", input);
        let hash = md5::compute(input);
        // println!("hash = {:x}", hash);
        let digit = hash[2] & 0x0F;
        if (hash[0] == 0) && (hash[1] == 0) && (hash[2] & 0xF0 == 0) {
            println!("{:x}", digit);
            count += 1;
            if count == 8 {
                break;
            }
        }
    }

    let mut count = 0;
    let mut output: [Option<u8>; 8] = [None; 8];
    for i in 1..99999999 {
        // println!("i = {}", i);
        let input = format!("{}{}", DOOR_ID, i);
        // println!("input = {}", input);
        let hash = md5::compute(input);
        // println!("hash = {:x}", hash);
        let digit = (hash[3] & 0xF0) >> 4;
        let pos: usize = hash[2] as usize & 0x0F;
        if (hash[0] == 0) && (hash[1] == 0) && (hash[2] & 0xF0 == 0) && (pos < 8) {
            println!("{:x} in pos {:x}", digit, pos);
            match output[pos] {
                None => {
                    output[pos] = Some(digit);
                    count += 1;
                }
                Some(_) => {}
            }
            if count == 8 {
                break;
            }
        }
    }
    println!(
        "Output: {}",
        output
            .iter()
            .map(|d| format!("{:x}", d.unwrap()))
            .collect::<String>()
    );
}
