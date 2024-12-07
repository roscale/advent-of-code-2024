use std::ops::{Add, Mul};
use itertools::Itertools;
use rayon::prelude::*;

fn total_calibration_result(
    equations: &[(usize, Vec<usize>)],
    operations: &[fn(x: usize, y: usize) -> usize],
) -> usize {
    equations.par_iter().filter_map(|(test_value, terms)| {
        (0..terms.len())
            .map(|_| operations)
            .multi_cartesian_product()
            .find_map(|ops| {
                let mut i = 0;
                let result = terms.iter().copied().reduce(|result, term| {
                    let result = ops[i](result, term);
                    i += 1;
                    result
                }).unwrap();
                if result == *test_value { Some(*test_value) } else { None }
            })
    }).sum()
}

fn main() {
    let input = include_str!("input.txt");

    let equations = input
        .lines()
        .map(|line| {
            let (test_value, terms) = line.split_once(": ").unwrap();
            (
                test_value.parse::<usize>().unwrap(),
                terms.split(" ").map(|term| term.parse::<usize>().unwrap()).collect_vec(),
            )
        }).collect_vec();

    println!("Part 1: {}", total_calibration_result(&equations, &[Add::add, Mul::mul]));

    fn concat(a: usize, b: usize) -> usize { a * 10_usize.pow(b.ilog10() + 1) + b }
    println!("Part 2: {}", total_calibration_result(&equations, &[Add::add, Mul::mul, concat]));
}
