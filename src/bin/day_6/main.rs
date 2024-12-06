use rayon::prelude::*;
use std::collections::HashMap;

#[derive(Copy, Clone)]
enum GuardState {
    Inside,
    Escaped,
    Stuck,
}

#[derive(Clone)]
struct Guard {
    position: (isize, isize),
    direction: (isize, isize),
    distinct_positions: HashMap<(isize, isize), (isize, isize)>, // position -> direction
    state: GuardState,
}

impl Guard {
    fn new(x: isize, y: isize) -> Self {
        Self {
            position: (x, y),
            direction: (0, -1),
            distinct_positions: HashMap::new(),
            state: GuardState::Inside,
        }
    }

    fn step(&mut self, grid: &Vec<Vec<char>>) {
        if let Some(&direction) = self.distinct_positions.get(&self.position) {
            if direction == self.direction {
                self.state = GuardState::Stuck;
                return;
            }
        }

        self.distinct_positions.entry(self.position)
            .or_insert(self.direction);

        let next_pos = (
            self.position.0 + self.direction.0,
            self.position.1 + self.direction.1,
        );

        match at(grid, next_pos.0, next_pos.1) {
            Some('#') | Some('O') => {
                self.direction = match self.direction {
                    (0, -1) => (1, 0),
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    _ => unreachable!(),
                };
            }
            None => self.state = GuardState::Escaped,
            _ => self.position = next_pos,
        };
    }
}

fn at(grid: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }
    grid.get(y as usize).and_then(|row|
        row.get(x as usize).copied())
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starting_pos = (|| {
        for (y, row) in grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == '^' {
                    return (x as isize, y as isize);
                }
            }
        }
        unreachable!();
    })();

    let mut guard = Guard::new(starting_pos.0, starting_pos.1);
    while let GuardState::Inside = guard.state {
        guard.step(&grid);
    }

    println!("Distinct positions: {}", guard.distinct_positions.len());

    let distinct_positions = guard.distinct_positions.keys()
        .filter(|&&pos| pos != starting_pos)
        .copied()
        .collect::<Vec<_>>();

    let obstacles: usize = distinct_positions.par_iter().map(|&pos| {
        let mut grid = grid.clone();
        grid[pos.1 as usize][pos.0 as usize] = 'O';

        let mut guard = Guard::new(starting_pos.0, starting_pos.1);
        while let GuardState::Inside = guard.state {
            guard.step(&grid);
        }

        if let GuardState::Stuck = guard.state { 1 } else { 0 }
    }).sum();

    println!("Obstacles: {}", obstacles);
}
