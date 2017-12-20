use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let input: usize = contents[0][0].parse().unwrap();
    let mut spinlocks: Vec<u32> = vec![0];
    let mut position = 0usize;
    for i in 1u32..2018u32 {
        position = (position + input) % spinlocks.len();
        spinlocks.insert(position + 1, i);
        position += 1;
    }
    println!("Part1: {}", spinlocks[position + 1]);

    let mut spinlocks_len = 1usize;
    let mut position = 0usize;
    let mut next = 0;
    for i in 1u32..50_000_000u32 {
        position = (position + input) % spinlocks_len;
        if position == 0 {
            next = i;
        }
        position += 1;
        spinlocks_len += 1;
    }
    println!("Next: {}", next);
    Ok(())
}
