use itertools::Itertools;
use regex::Regex;

fn main() {
    let robot_regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let input = include_str!("input.txt");
    let (width, height) = (101, 103);

    let mut robots = input.lines().map(|line| {
        let captures = robot_regex.captures(line).unwrap();
        let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let vx = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let vy = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();
        (x, y, vx, vy)
    }).collect::<Vec<_>>();

    let safety_factor: u32 = robots.iter()
        .map(|(x, y, vx, vy)| {
            ((x + 100 * vx).rem_euclid(width), (y + 100 * vy).rem_euclid(height))
        })
        .fold([0, 0, 0, 0], |quadrants, robot| {
            let half_width = width as f32 / 2.0;
            let half_height = height as f32 / 2.0;

            let left = robot.0 < half_width.floor() as i32;
            let right = robot.0 >= half_width.ceil() as i32;
            let top = robot.1 < half_height.floor() as i32;
            let bottom = robot.1 >= half_height.ceil() as i32;

            match (left, right, top, bottom) {
                (true, false, true, false) => [quadrants[0] + 1, quadrants[1], quadrants[2], quadrants[3]],
                (false, true, true, false) => [quadrants[0], quadrants[1] + 1, quadrants[2], quadrants[3]],
                (true, false, false, true) => [quadrants[0], quadrants[1], quadrants[2] + 1, quadrants[3]],
                (false, true, false, true) => [quadrants[0], quadrants[1], quadrants[2], quadrants[3] + 1],
                _ => quadrants,
            }
        }).iter().product();

    println!("Part 1: {}", safety_factor);

    for step in 1.. {
        for (x, y, vx, vy) in &mut robots {
            *x = (*x + *vx).rem_euclid(width);
            *y = (*y + *vy).rem_euclid(height);
        }
        robots.sort();

        let mut contiguous = 1;
        for ((_, y1, ..), (_, y2, ..)) in robots.iter().tuple_windows() {
            if *y2 == y1 + 1 {
                contiguous += 1;
                if contiguous >= 30 {
                    println!("Part 2: {}", step);
                    return;
                }
            } else {
                contiguous = 1;
            }
        }
    }
}
