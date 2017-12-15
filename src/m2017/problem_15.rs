pub fn run(_contents: &Vec<Vec<String>>) {
    const FACTOR_A: u64 = 16807;
    const FACTOR_B: u64 = 48271;
    const LIMIT: u64 = 2147483647;

    let mut count = 0;
    let mut a = 873;
    let mut b = 583;
    for _ in 0..40_000_000 {
        a = (a * FACTOR_A) % LIMIT;
        b = (b * FACTOR_B) % LIMIT;
        if (a & 0xFFFF) == (b & 0xFFFF) {
            count = count + 1;
        }
    }
    println!("Count: {}", count);

    let mut count = 0;
    let mut a = 873;
    let mut b = 583;
    for _ in 0..5_000_000 {
        a = (a * FACTOR_A) % LIMIT;
        while (a % 4) != 0 {
            a = (a * FACTOR_A) % LIMIT;
        }
        b = (b * FACTOR_B) % LIMIT;
        while (b % 8) != 0 {
            b = (b * FACTOR_B) % LIMIT;
        }
        if (a & 0xFFFF) == (b & 0xFFFF) {
            count = count + 1;
        }
    }
    println!("Count: {}", count);

}
