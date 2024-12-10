use std::collections::HashSet;

trait Grid<T> {
    fn at(&self, pos: (isize, isize)) -> Option<T>;
    fn grid_iter(&self) -> GridIterator<T>;
}

impl<T: Copy> Grid<T> for Vec<Vec<T>> {
    fn at(&self, (x, y): (isize, isize)) -> Option<T> {
        if x < 0 || y < 0 {
            return None;
        }
        self.get(x as usize).and_then(|row|
            row.get(y as usize).copied())
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

fn explore(grid: &Vec<Vec<u8>>, start: (isize, isize), distinct: bool) -> usize {
    let mut stack = vec![start];
    let mut discovered = HashSet::new();
    let mut trails = 0;

    while let Some(pos) = stack.pop() {
        discovered.insert(pos);

        let height = grid.at(pos).unwrap();
        if height == 9 {
            trails += 1;
            continue;
        }

        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_pos = (pos.0 + direction.0, pos.1 + direction.1);

            if let Some(next_height) = grid.at(next_pos) {
                if next_height == height + 1 && (distinct || !discovered.contains(&next_pos)) {
                    stack.push(next_pos);
                    discovered.insert(next_pos);
                }
            }
        }
    }

    trails
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines()
        .map(|line| line.chars()
            .map(|digit| digit.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let trailheads = grid.grid_iter()
        .filter(|&(cell, _)| cell == 0)
        .collect::<Vec<_>>();

    let score: usize = trailheads.iter().map(|(_, position)| explore(&grid, *position, false)).sum();

    println!("Score: {}", score);

    let rating: usize = trailheads.iter().map(|(_, position)| explore(&grid, *position, true)).sum();

    println!("Rating: {}", rating);
}
