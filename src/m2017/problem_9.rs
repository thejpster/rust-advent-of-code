pub fn run(contents: &Vec<Vec<String>>) {
    for line in &contents[0] {
        let mut skip = false;
        let mut garbage = false;
        let mut level = 0;
        let mut total = 0;
        let mut garbage_count = 0;
        for ch in line.chars() {
            if skip {
                skip = false;
                continue;
            }
            match ch {
                '!' if garbage => skip = true,
                '>' if garbage => garbage = false,
                _ if garbage => garbage_count = garbage_count + 1,
                '{' => {
                    level = level + 1;
                    total = total + level;
                }
                '}' => {
                    level = level - 1;
                }
                '<' => garbage = true,
                '!' => panic!("Can't skip outside garbage!"),
                '>' => panic!("Can't end garbage if it didn't start!"),
                _ => {}
            }
        }
        println!("Total: {}, Garbage Count: {}", total, garbage_count);
    }
}
