use std::collections::HashMap;

pub fn run(contents: &Vec<Vec<String>>) {
    let mut registers = HashMap::new();
    let mut highest = ("x", 0);
    for line in &contents[0] {
        let parts: Vec<&str> = line.split_whitespace().collect();
        // y inc 497 if n <= 3
        let register = parts[0];
        let op = parts[1];
        let delta = parts[2].parse::<i64>().unwrap();
        assert_eq!(parts[3], "if");
        let check_register = parts[4];
        let check_op = parts[5];
        let check_value = parts[6].parse::<i64>().unwrap();

        let reg_contents = *registers.get(check_register).unwrap_or(&0);
        let ok = match check_op {
            ">" => reg_contents > check_value,
            ">=" => reg_contents >= check_value,
            "<=" => reg_contents <= check_value,
            "<" => reg_contents < check_value,
            "!=" => reg_contents != check_value,
            "==" => reg_contents == check_value,
            _ => panic!("Bad test {}!", line),
        };
        if ok {
            let old = *registers.get(register).unwrap_or(&0);
            let new = match op {
                "inc" => old + delta,
                "dec" => old - delta,
                _ => panic!("Bad operation {}!", op),
            };
            registers.insert(register, new);
            if new > highest.1 {
                highest = (register, new);
            }
        }
    }
    let max = registers.iter().max_by_key(|x| x.1).unwrap();
    println!("Max: {:?}", max);
    println!("Highest: {:?}", highest);
}
