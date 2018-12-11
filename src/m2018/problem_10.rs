use failure::Error;
use std::collections::HashSet;
pub fn run(contents: &[Vec<String>]) -> Result<(), Error> {
    let re = regex::Regex::new(
        r"position=<\s*([\-\d]+),\s*([\-\d]+)> velocity=<\s*([\-\d]+),\s*([\-\d]+)>",
    )
    .unwrap();
    let mut input: Vec<((i32, i32), (i32, i32))> = contents[0]
        .iter()
        .map(|line| {
            let values = re.captures(line).expect("match failure");
            (
                (
                    values.get(1).unwrap().as_str().parse().unwrap(),
                    values.get(2).unwrap().as_str().parse().unwrap(),
                ),
                (
                    values.get(3).unwrap().as_str().parse().unwrap(),
                    values.get(4).unwrap().as_str().parse().unwrap(),
                ),
            )
        })
        .collect();

    let mut times: Vec<(i32, i32)> = input
        .iter()
        .map(|point| {
            let time_x = (point.0).0 / (point.1).0;
            let time_y = (point.0).1 / (point.1).1;
            (time_x, time_y)
        })
        .collect();

    times.sort_by_key(|x| x.0);

    println!("times: {:?}", times);

    let mut old_width = 100_000_000;
    let mut old_points = HashSet::new();
    for step in 0..10500 {
        let mut min_x = 999_999;
        let mut max_x = -999_999;
        let mut points = HashSet::new();
        for point in input.iter_mut() {
            (point.0).0 += (point.1).0;
            (point.0).1 += (point.1).1;
            min_x = min_x.min((point.0).0);
            max_x = max_x.max((point.0).0);
            points.insert(point.0);
        }
        let width = (max_x - min_x).abs();
        println!("Width @ {}: {} {} {}", step, width, max_x, min_x);
        if width > old_width {
            println!("Mid width: {} @ {}", width, step);
            for y in 120..200 {
                for x in 0..200 {
                    let p = (x, y);
                    if old_points.contains(&p) {
                        print!("#");
                    } else {
                        print!(" ");
                    }
                }
                println!();
            }
            break;
        }
        old_width = width;
        old_points = points;
    }

    Ok(())
}
