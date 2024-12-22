#![feature(iter_map_windows)]

use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::BitXor;

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

fn diff_sequence(prices: &[usize; 5]) -> [isize; 4] {
    [
        prices[1] as isize - prices[0] as isize,
        prices[2] as isize - prices[1] as isize,
        prices[3] as isize - prices[2] as isize,
        prices[4] as isize - prices[3] as isize,
    ]
}

fn diff_to_price(seed: usize) -> HashMap<[isize; 4], usize> {
    let secret_numbers = iter::once(seed).chain(rand_iterator(seed).take(2000));

    let diff_and_price = secret_numbers.map_windows(|five_prices| {
        let diff = diff_sequence(five_prices);
        let price = five_prices.last().unwrap() % 10;
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
