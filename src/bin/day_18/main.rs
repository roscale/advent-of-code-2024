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
        let cell = self.grid.at((self.x, self.y))?;
        let ret = Some((cell, (self.x, self.y)));

        self.x += 1;
        if self.x >= self.grid[0].len() as isize {
            self.x = 0;
            self.y += 1;
        }

        ret
    }
}

fn dijkstra(grid: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> Option<isize> {
    let mut queue = Vec::new();
    let mut costs = HashMap::new();

    costs.insert(start, 0);
    queue.push(start);

    while let Some(pos) = queue.pop() {
        let cost = costs[&pos];

        for next_direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);

            if !matches!(grid.at(next), Some('.')) {
                continue;
            }

            let next_cost = cost + 1;

            if next_cost < *costs.entry(next).or_insert(isize::MAX) {
                queue.push(next);
                costs.insert(next, next_cost);
            }
        }
    }
    costs.get(&end).copied()
}

fn can_reach(grid: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> bool {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();

    queue.push(start);

    while let Some(pos) = queue.pop() {
        if pos == end {
            return true;
        }

        for next_direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);

            if !matches!(grid.at(next), Some('.')) {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }

            visited.insert(next);
            queue.push(next);
        }
    }

    false
}

fn main() {
    let input = include_str!("input.txt");

    let coords = input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap())
    }).collect::<Vec<_>>();

    let mut memory = vec![vec!['.'; 71]; 71];
    for (x, y) in coords.iter().take(1024) {
        memory[*x as usize][*y as usize] = '#';
    }

    let steps = dijkstra(&memory, (0, 0), (70, 70));
    println!("Part 1: {}", steps.unwrap());

    for (x, y) in coords.iter().skip(1024) {
        memory[*x as usize][*y as usize] = '#';
        if can_reach(&memory, (0, 0), (70, 70)) {
            println!("Part 2: {},{}", x, y);
            break;
        }
    }
}
