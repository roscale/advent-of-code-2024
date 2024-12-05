use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("input.txt");
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let mut precedence: HashMap<usize, HashSet<usize>> = HashMap::new();

    rules.split_whitespace().for_each(|rule| {
        let (left, right) = rule.split_once('|').unwrap();
        let (left, right) = (left.parse().unwrap(), right.parse().unwrap());
        precedence.entry(left).or_default().insert(right);
    });

    let mut full_precedence: HashMap<usize, HashSet<usize>> = HashMap::new();

    fn propagate(
        smaller: usize,
        precedence: &mut HashMap<usize, HashSet<usize>>,
        full_precedence: &mut HashMap<usize, HashSet<usize>>,
    ) {
        let Some(bigger) = precedence.remove(&smaller) else { return };
        for &b in bigger.iter() {
            propagate(b, precedence, full_precedence);
        }
        full_precedence.entry(smaller).or_default().extend(bigger);
    }

    let keys = precedence.keys().copied().collect::<Vec<_>>();
    for smaller in keys {
        propagate(smaller, &mut precedence, &mut full_precedence);
    }

    let updates: Vec<Vec<usize>> = updates.lines().map(|update|
        update.split(',').map(|page|
            page.parse().unwrap()
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>();

    let (correct_updates, mut incorrect_updates): (Vec<_>, Vec<_>) = updates.into_iter().partition(|update|
        update.is_sorted_by(|a, b|
            full_precedence
                .get(a)
                .and_then(|set| set.get(b))
                .is_some()
        )
    );

    let sum: usize = correct_updates.iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Correct: {}", sum);

    for update in incorrect_updates.iter_mut() {
        update.sort_by(|a, b| {
            let smaller = full_precedence
                .get(a)
                .and_then(|set| set.get(b))
                .is_some();

            if smaller {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });
    }

    let sum: usize = incorrect_updates.iter()
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Incorrect: {}", sum);
}
