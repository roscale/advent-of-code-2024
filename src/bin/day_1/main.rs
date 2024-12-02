use itertools::Itertools;

fn main() {
    let input = include_str!("input.txt");

    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(left, right)| (
            left.parse::<usize>().unwrap(),
            right.parse::<usize>().unwrap(),
        ))
        .unzip();

    left.sort();
    right.sort();

    let distance: usize = left.iter()
        .zip(right.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    println!("Distance: {}", distance);

    let left = left.into_iter()
        .into_grouping_map_by(|&id| id)
        .fold(0, |count, _, _| count + 1);

    let right = right.into_iter()
        .into_grouping_map_by(|&id| id)
        .fold(0, |count, _, _| count + 1);

    let similarity: usize = left.into_iter()
        .map(|(id, left_count)| {
            let right_count = right.get(&id).unwrap_or(&0);
            id * left_count * right_count
        })
        .sum();

    println!("Similarity: {}", similarity);
}
