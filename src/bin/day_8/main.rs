use itertools::Itertools;
use std::collections::HashSet;
use std::iter;
use std::iter::Scan;
use std::ops::RangeFrom;

trait Grid {
    fn at(&self, pos: (isize, isize)) -> Option<char>;
    fn grid_iter(&self) -> GridIterator;
}

impl Grid for Vec<Vec<char>> {
    fn at(&self, (x, y): (isize, isize)) -> Option<char> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize).and_then(|row| row.get(y as usize).copied())
    }

    fn grid_iter(&self) -> GridIterator {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

struct GridIterator<'a> {
    grid: &'a Vec<Vec<char>>,
    x: isize,
    y: isize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = (char, (isize, isize));

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cell) = self.grid.at((self.x, self.y)) else {
            return None;
        };

        let ret = Some((cell, (self.x, self.y)));

        self.x += 1;
        if self.x >= self.grid[0].len() as isize {
            self.x = 0;
            self.y += 1;
        }

        ret
    }
}

fn antinodes_in_direction<'a>(
    grid: &'a Vec<Vec<char>>,
    start: (isize, isize),
    direction: (isize, isize),
) -> impl Iterator<Item=(isize, isize)> + 'a {
    (0..).scan(start, move |pos, _| {
        pos.0 += direction.0;
        pos.1 += direction.1;
        grid.at(*pos).map(|_| *pos)
    })
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines().map(|line| line.chars().collect_vec()).collect_vec();

    let antennas = grid.grid_iter()
        .filter(|&(cell, _)| cell != '.')
        .into_group_map_by(|(cell, _)| *cell);

    let antenna_combinations = antennas.iter()
        .map(|(_, positions)|
            positions.iter()
                .map(|pos| pos.1)
                .combinations(2)
                .map(|pair| (pair[0], pair[1])))
        .flatten()
        .collect_vec();

    let mut antinodes = HashSet::new();

    for pair in &antenna_combinations {
        let first = pair.0;
        let second = pair.1;

        let direction = (second.0 - first.0, second.1 - first.1);
        let opposite_direction = (-direction.0, -direction.1);

        if let Some(antinode) = antinodes_in_direction(&grid, second, direction).next() {
            antinodes.insert(antinode);
        }

        if let Some(antinode) = antinodes_in_direction(&grid, first, opposite_direction).next() {
            antinodes.insert(antinode);
        }
    }

    println!("Part 1: {}", antinodes.len());

    for pair in &antenna_combinations {
        let first = pair.0;
        let second = pair.1;

        antinodes.insert(first);
        antinodes.insert(second);

        let direction = (second.0 - first.0, second.1 - first.1);
        let opposite_direction = (-direction.0, -direction.1);

        for antinode in antinodes_in_direction(&grid, second, direction) {
            antinodes.insert(antinode);
        }

        for antinode in antinodes_in_direction(&grid, first, opposite_direction) {
            antinodes.insert(antinode);
        }
    }

    println!("Part 2: {}", antinodes.len());
}
