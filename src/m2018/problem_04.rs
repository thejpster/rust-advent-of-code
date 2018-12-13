use failure::Error;
use std::collections::HashMap;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let mut guards: HashMap<u32, HashMap<u8, usize>> = HashMap::new();
    let mut last_guard: u32 = 0;
    let mut last_asleep = 0;

    let mut vec = contents[0].clone();

    vec.sort();

    for line in vec.iter() {
        println!("Line: {}", line);
        let parts: Vec<&str> = line.split(" ").collect();
        if line.contains("Guard #") {
            println!("Parsing guard {:?}", &parts[3][1..]);
            last_guard = parts[3][1..].parse()?;
        }
        if line.contains("falls asleep") {
            println!("Parsing minutes {:?}", &parts[1][3..5]);
            let minutes = parts[1][3..5].parse()?;
            last_asleep = minutes;
        }
        if line.contains("wakes up") {
            println!("Parsing minutes {:?}", &parts[1][3..5]);
            let wake_at = parts[1][3..5].parse()?;

            let mut guard_map = guards.entry(last_guard).or_insert(HashMap::new());
            for i in last_asleep..wake_at {
                *guard_map.entry(i).or_insert(0) += 1;
            }
        }
    }

    let mut max_asleep = (0, 0);
    for (guard, map) in guards.iter() {
        let total_asleep: usize = map.values().sum();
        println!("Guard {} spent {} minutes asleep", guard, total_asleep);
        if total_asleep > max_asleep.0 {
            max_asleep = (total_asleep, *guard);
        }
    }
    println!("max_asleep = {:?}", max_asleep);

    let mut best_minute = (0, 0);
    for (&minute, &count) in guards[&max_asleep.1].iter() {
        if count > best_minute.0 {
            best_minute = (count, minute);
        }
    }
    println!("best_minute = {:?}", best_minute);

    let mut best_minute2 = (0, 0, 0);
    for (guard, map) in guards.iter() {
        for (&minute, &count) in map.iter() {
            if count > best_minute2.0 {
                best_minute2 = (count, minute, *guard);
            }
        }
    }
    println!("best_minute2 = {:?}", best_minute2);

    println!("Answer 1: {}", best_minute.1 as u32 * max_asleep.1);
    println!("Answer 2: {}", best_minute2.1 as u32 * best_minute2.2);
    Ok(())
}
