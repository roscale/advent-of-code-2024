use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::BitXor;
use itertools::Itertools;

fn next(n: usize) -> usize {
    let n = n.bitxor(n * 64) % 16777216;
    let n = n.bitxor(n >> 5) % 16777216;
    n.bitxor(n * 2048) % 16777216
}

fn rand_iterator(seed: usize) -> impl Iterator<Item=usize> {
    (0..).scan(seed, |n, _| {
        *n = next(*n);
        Some(*n)
    })
}

fn diff_sequence(a: usize, b: usize, c: usize, d: usize, e: usize) -> (isize, isize, isize, isize) {
    (
        (b % 10) as isize - (a % 10) as isize,
        (c % 10) as isize - (b % 10) as isize,
        (d % 10) as isize - (c % 10) as isize,
        (e % 10) as isize - (d % 10) as isize,
    )
}

fn diff_to_price(seed: usize) -> HashMap<(isize, isize, isize, isize), usize> {
    let secret_numbers = iter::once(seed).chain(rand_iterator(seed).take(2000));
    let five_sliding_window = secret_numbers.tuple_windows::<(_, _, _, _, _)>();

    let diff_and_price = five_sliding_window.map(|(a, b, c, d, e)| {
        let diff = diff_sequence(a, b, c, d, e);
        let price = e % 10;
        (diff, price)
    });

    let mut diff_to_price = HashMap::new();
    for (diff, price) in diff_and_price {
        diff_to_price.entry(diff).or_insert(price);
    }

    diff_to_price
}

fn main() {
    let input = include_str!("input.txt");

    let seeds = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let sum: usize = seeds.iter()
        .map(|&seed| rand_iterator(seed).take(2000).last().unwrap())
        .sum();

    println!("Part 1: {}", sum);

    let seed_to_diff_to_price = seeds.iter().copied()
        .map(diff_to_price)
        .collect::<Vec<_>>();

    let mut all_diffs = HashSet::new();
    for diff_to_price in &seed_to_diff_to_price {
        all_diffs.extend(diff_to_price.keys().copied());
    }

    let diff_to_bananas = |diff| {
        seed_to_diff_to_price.iter()
            .map(|prices| prices.get(diff).unwrap_or(&0))
            .sum::<usize>()
    };

    let most_bananas = all_diffs.iter()
        .map(diff_to_bananas)
        .max().unwrap();

    println!("Part 2: {:?}", most_bananas);
}
