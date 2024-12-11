use std::collections::HashMap;

fn number_of_digits(n: usize) -> u32 {
    n.checked_ilog10().unwrap_or(0) + 1
}

fn split_in_half(n: usize) -> (usize, usize) {
    let half = number_of_digits(n) / 2;
    let left = n / 10_usize.pow(half);
    let right = n % 10_usize.pow(half);
    (left, right)
}

fn expand(
    stone: usize,
    depth: usize,
    max_depth: usize,
    cache: &mut HashMap<usize, HashMap<usize, usize>>,
) -> usize {
    if depth == max_depth {
        return 1;
    }
    if let Some(Some(&count)) = cache.get(&stone).map(|tree| tree.get(&depth)) {
        return count;
    }

    let count = if stone == 0 {
        expand(1, depth + 1, max_depth, cache)
    } else if number_of_digits(stone) % 2 == 0 {
        let (left, right) = split_in_half(stone);
        expand(left, depth + 1, max_depth, cache) +
            expand(right, depth + 1, max_depth, cache)
    } else {
        expand(stone * 2024, depth + 1, max_depth, cache)
    };

    cache.entry(stone).or_default().insert(depth, count);
    count
}

fn main() {
    let input = include_str!("input.txt");

    let stones = input.split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut cache = HashMap::new();
    let count: usize = stones.iter()
        .map(|&stone| expand(stone, 0, 25, &mut cache))
        .sum();

    println!("Part 1: {}", count);

    let mut cache = HashMap::new();
    let count: usize = stones.iter()
        .map(|&stone| expand(stone, 0, 75, &mut cache))
        .sum();

    println!("Part 2: {}", count);
}
