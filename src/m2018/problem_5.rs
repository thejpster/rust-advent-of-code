use failure::Error;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    println!("Answer 1: {}", reduce(&contents[0][0]));

    for remove in "abcdefghijklmnopqrstuvwxyz".chars() {
        let test = contents[0][0]
            .replace(remove, "")
            .replace(remove.to_ascii_uppercase(), "");
        println!("{} -> {}", remove, reduce(&test));
    }

    Ok(())
}

fn reduce(input: &str) -> usize {
    let mut new_chain = input.to_owned();
    let mut changes = true;
    while changes {
        changes = false;
        let chain = new_chain.clone();
        for ((i, a), b) in chain.chars().enumerate().zip(chain[1..].chars()) {
            if a.is_lowercase() && b.is_uppercase() && (a == b.to_ascii_lowercase()) {
                new_chain = chain[0..i].to_owned() + &chain[i + 2..];
                changes = true;
                break;
            }
            if a.is_uppercase() && b.is_lowercase() && (a.to_ascii_lowercase() == b) {
                new_chain = chain[0..i].to_owned() + &chain[i + 2..];
                changes = true;
                break;
            }
        }
    }
    new_chain.len()
}
