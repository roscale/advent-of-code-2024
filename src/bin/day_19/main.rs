#![allow(unstable_name_collisions)]

use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

fn match_count<'a>(patterns: &[&str], design: &'a str, cache: &mut HashMap<&'a str, usize>) -> usize {
    patterns.iter()
        .filter(|&&pattern| design.starts_with(pattern))
        .map(|&pattern| {
            if design == pattern {
                return 1;
            }

            let rest = &design[pattern.len()..];
            if let Some(&count) = cache.get(rest) {
                return count;
            }

            let count = match_count(patterns, rest, cache);
            cache.insert(rest, count);
            count
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");

    let (patterns, designs) = input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();

    let or = patterns.iter().intersperse(&"|").copied().collect::<String>();
    let regex = Regex::new(&format!("^({})+$", or)).unwrap();

    let count = designs.iter().filter(|design| regex.is_match(design)).count();
    println!("Part 1: {}", count);

    let mut cache = HashMap::new();

    let count: usize = designs.iter()
        .map(|design| match_count(&patterns, design, &mut cache))
        .sum();

    println!("Part 2: {}", count);
}
