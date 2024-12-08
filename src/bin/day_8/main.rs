use itertools::Itertools;
use std::collections::HashSet;
use std::iter;

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
        let antinode_1 = (second.0 + direction.0, second.1 + direction.1);
        let antinode_2 = (first.0 - direction.0, first.1 - direction.1);

        if grid.at(antinode_1).is_some() {
            antinodes.insert(antinode_1);
        }

        if grid.at(antinode_2).is_some() {
            antinodes.insert(antinode_2);
        }
    }

    println!("Part 1: {}", antinodes.len());

    for pair in &antenna_combinations {
        let first = pair.0;
        let second = pair.1;

        antinodes.insert(first);
        antinodes.insert(second);

        let direction = (second.0 - first.0, second.1 - first.1);

        let mut pos = second;
        let one_way = iter::repeat_with(|| {
            let antinode = (pos.0 + direction.0, pos.1 + direction.1);
            pos = antinode;
            grid.at(antinode).map(|_| antinode)
        });

        for antinode in one_way.while_some() {
            antinodes.insert(antinode);
        }

        let mut pos = first;
        let the_other_way = iter::repeat_with(|| {
            let antinode = (pos.0 - direction.0, pos.1 - direction.1);
            pos = antinode;
            grid.at(antinode).map(|_| antinode)
        });

        for antinode in the_other_way.while_some() {
            antinodes.insert(antinode);
        }
    }

    println!("Part 2: {}", antinodes.len());
}
