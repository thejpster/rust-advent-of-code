use failure::Error;
pub fn run(_contents: &[Vec<String>]) -> Result<(), Error> {
    let num_players = 424;
    let max_marble = 71144 * 100;

    let mut circle: Vec<u32> = Vec::new();
    let mut players: Vec<u32> = vec![0; num_players];
    let mut current_idx = 0;
    let mut player_idx = 0;

    circle.push(0);

    for next_marble in 1..=max_marble {
        if (next_marble % 23) == 0 {
            players[player_idx] += next_marble;
            // println!("Kept {}", next_marble);
            let mut remove_idx = current_idx as i32 - 7;
            while remove_idx < 0 {
                remove_idx += circle.len() as i32;
            }
            let removed = circle.remove(remove_idx as usize);
            players[player_idx] += removed;
            // println!("Removed {}", removed);
            current_idx = remove_idx as usize;
        } else {
            let new_idx = ((current_idx + 1) % circle.len()) + 1;
            // println!("Added {} at {}", next_marble, new_idx);
            circle.insert(new_idx, next_marble);
            current_idx = new_idx;
        }
        player_idx += 1;
        if player_idx == players.len() {
            player_idx = 0;
        }
        // println!("Circle: {:?}", circle);
    }
    println!("Scores: {:?}", players);
    println!("Highest Score: {:?}", players.iter().max());
    Ok(())
}
