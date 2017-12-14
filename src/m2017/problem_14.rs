use std::collections::HashSet;
use super::problem_10::calculate;

const MAX: u8 = 255;

type Position = (i32, i32);
type Board = HashSet<Position>;

pub fn run(_contents: &Vec<Vec<String>>) {
    let mut items: Vec<u8> = (0..MAX).collect();
    // Don't have inclusive range syntax, so manually push on the last item
    items.push(MAX);

    let mut board: Board = Board::new();
    for line in 0..128 {
        let key = format!("ffayrhll-{}", line);
        let hash = calculate(&mut items.clone(), &key);
        for (byte_idx, byte) in hash.iter().enumerate() {
            for idx in 0..8 {
                let shift = 7 - idx;
                let bit = (byte >> shift) & 1u8;
                print!("{}", if bit == 0u8 { "." } else { "#" });
                if bit != 0u8 {
                    let p = (line, (byte_idx as i32 * 8) + idx);
                    board.insert(p);
                }
            }
        }
        println!();
    }
    println!("Count: {}", board.len());

    let mut regions = 0;
    while let Some(p) = find_unset(&board) {
        regions = regions + 1;
        fill(&mut board, p);
    }
    println!("Regions: {}", regions);
}

fn find_unset(board: &Board) -> Option<Position> {
    return board.iter().next().cloned()
}

fn fill(board: &mut Board, pos: Position) {
    if board.remove(&pos) {
        for shift in [(-1, 0), (1, 0), (0, -1), (0, 1)].iter() {
            let tpos = (shift.0 + pos.0, shift.1 + pos.1);
            fill(board, tpos);
        }
    }
}
