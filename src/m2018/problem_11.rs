use failure::Error;
use std::collections::HashSet;
pub fn run(_contents: &[Vec<String>]) -> Result<(), Error> {
    let grid_serial_number = 2866;
    let mut max = ((0, 0), 0);
    for left in 1..=298 {
        for top in 1..=298 {
            let powers = [
                cell_power(left, top, grid_serial_number),
                cell_power(left + 1, top, grid_serial_number),
                cell_power(left + 2, top, grid_serial_number),
                cell_power(left, top + 1, grid_serial_number),
                cell_power(left + 1, top + 1, grid_serial_number),
                cell_power(left + 2, top + 1, grid_serial_number),
                cell_power(left, top + 2, grid_serial_number),
                cell_power(left + 1, top + 2, grid_serial_number),
                cell_power(left + 2, top + 2, grid_serial_number),
            ];
            let total = powers.iter().sum();
            if total > max.1 {
                max = ((left, top), total);
            }
        }
    }

    let mut max = ((0, 0, 0), 0);
    for size in 1..=300 {
        println!("Size {}", size);
        for left in 1..=(301 - size) {
            for top in 1..=(301 - size) {
                let mut total = 0;
                for square_x in 0..size {
                    for square_y in 0..size {
                        total += cell_power(left + square_x, top + square_y, grid_serial_number);
                    }
                }
                // println!("{:?}", ((left, top, size), total));
                if total > max.1 {
                    max = ((left, top, size), total);
                }
            }
        }
        println!("Max so far: {:?}", max);
    }

    println!("Max: {:?}", max);
    Ok(())
}

fn cell_power(x: i32, y: i32, gsn: i32) -> i32 {
    let rack_id = x + 10;
    let mut power = rack_id * y;
    power += gsn;
    power *= rack_id;
    power = (power / 100) % 10;
    power - 5
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cell_power() {
        assert_eq!(cell_power(122, 79, 57), -5);
        assert_eq!(cell_power(217, 196, 39), 0);
        assert_eq!(cell_power(101, 153, 71), 4);
    }
}
