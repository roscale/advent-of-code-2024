use std::collections::{HashMap, HashSet};
use std::iter;
use std::ops::BitXor;
use itertools::Itertools;

fn next(n: usize) -> usize {
    let n = n.bitxor(n * 64) % 16777216;
    let n = n.bitxor(n >> 5) % 16777216;
    n.bitxor(n * 2048) % 16777216
}

fn sequence(seed: usize) -> impl Iterator<Item=usize> {
    (0..2000).scan(seed, |n, _| {
        *n = next(*n);
        Some(*n)
    })
}

fn main() {
    let input = include_str!("input.txt");

    let seeds = input.lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let sum: usize = seeds.iter()
        .map(|&seed| {
            sequence(seed).last().unwrap()
        })
        .sum();

    println!("Part 1: {}", sum);

    let mut all_changes = HashSet::new();

    let prices = seeds.iter()
        .map(|&seed| {
            let five_prices = iter::once(seed).chain(sequence(seed)).tuple_windows::<(_, _, _, _, _)>();
            let changes = five_prices.map(|(a, b, c, d, e)| {
                let changes = (
                    (b % 10) as isize - (a % 10) as isize,
                    (c % 10) as isize - (b % 10) as isize,
                    (d % 10) as isize - (c % 10) as isize,
                    (e % 10) as isize - (d % 10) as isize,
                );
                (changes, e % 10)
            });

            let mut prices = HashMap::new();

            for (changes, price) in changes {
                prices.entry(changes).or_insert(price);
                all_changes.insert(changes);
            }
            
            prices
        })
        .collect::<Vec<_>>();

    let most_bananas = all_changes.iter()
        .map(|changes| {
            prices.iter()
                .map(|prices| {
                    prices.get(changes).unwrap_or(&0)
                })
                .sum::<usize>()
        })
        .max().unwrap();

    println!("Part 2: {:?}", most_bananas);
}
