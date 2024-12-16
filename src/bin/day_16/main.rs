use std::collections::{HashMap, HashSet};

trait Grid<T> {
    fn at(&self, pos: (isize, isize)) -> Option<T>;
    fn grid_iter(&self) -> GridIterator<T>;
}

impl<T: Copy> Grid<T> for Vec<Vec<T>> {
    fn at(&self, (x, y): (isize, isize)) -> Option<T> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(y as usize).and_then(|row|
            row.get(x as usize).copied())
    }

    fn grid_iter(&self) -> GridIterator<T> {
        GridIterator {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

struct GridIterator<'a, T> {
    grid: &'a Vec<Vec<T>>,
    x: isize,
    y: isize,
}

impl<'a, T: Copy> Iterator for GridIterator<'a, T> {
    type Item = (T, (isize, isize));

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

fn dijkstra(grid: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> (isize, isize) {
    let mut queue = Vec::new();
    let mut costs = HashMap::new();
    let mut previous = HashMap::new();
    let mut best_tiles: HashMap<isize, HashSet<(isize, isize)>> = HashMap::new();

    costs.insert(start, 0);
    queue.push((start, (1, 0)));

    while let Some((pos, direction)) = queue.pop() {
        let cost = costs[&pos];

        if pos == end {
            let mut p = end;
            best_tiles.entry(cost).or_default().insert(p);
            while p != start {
                p = previous[&p];
                best_tiles.entry(cost).or_default().insert(p);
            }
            continue;
        }

        for next_direction in [direction, (direction.1, direction.0), (-direction.1, -direction.0)] {
            let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);

            if !matches!(grid.at(next), Some('.' | 'E')) {
                continue;
            }

            let next_cost = cost + 1
                + if next_direction != direction { 1000 } else { 0 };

            if next_cost < *costs.entry(next).or_insert(isize::MAX) ||
                next_cost - 1000 <= *costs.entry(next).or_insert(isize::MAX) {
                queue.push((next, next_direction));
                costs.insert(next, next_cost);
                previous.insert(next, pos);
            }
        }
    }
    (costs[&end], best_tiles[&costs[&end]].len() as isize)
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines()
        .map(|line| line.chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = grid.grid_iter().find(|(cell, _)| *cell == 'S').unwrap().1;
    let end = grid.grid_iter().find(|(cell, _)| *cell == 'E').unwrap().1;

    let (cost, best_tiles) = dijkstra(&grid, start, end);
    println!("Part 1: {}", cost);
    println!("Part 2: {}", best_tiles);
}
