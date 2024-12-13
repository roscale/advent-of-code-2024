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

fn flood_fill(grid: &Vec<Vec<char>>, start: (isize, isize), visited: &mut HashSet<(isize, isize)>) -> (usize, usize, usize) {
    let letter = grid.at(start).unwrap();
    let mut stack = vec![start];
    let mut area = 0;
    let mut perimeter = 0;
    let mut corners = HashSet::new();
    let mut diagonals = 0;

    while let Some(pos) = stack.pop() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        area += 1;

        let mut edge = false;

        for direction in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = (pos.0 + direction.0, pos.1 + direction.1);
            if grid.at(next) == Some(letter) && visited.contains(&next) {
                continue;
            }
            if grid.at(next) == Some(letter) {
                stack.push(next);
            } else {
                perimeter += 1;
                edge = true;
            }
        }

        if !edge {
            continue;
        }

        for shift in [(-1, -1), (-1, 0), (0, -1), (0, 0)] {
            let quad_pos = [(0, 0), (0, 1), (1, 0), (1, 1)]
                .map(|(x, y)| (pos.0 + shift.0 + x, pos.1 + shift.1 + y));

            let quad = quad_pos.map(|quad_pos| grid.at(quad_pos));

            if (quad[1] != Some(letter) && quad[2] != Some(letter)) ||
                (quad[0] != Some(letter) && quad[3] != Some(letter)) {
                diagonals += 1;
            } else {
                let sum = quad.iter().filter(|&&c| c == Some(letter)).count();
                if sum % 2 != 0 {
                    corners.insert(quad_pos);
                }
            }
        }
    }

    (area, perimeter, corners.len() + diagonals)
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();

    let (cost_1, cost_2) = grid.grid_iter().fold((0, 0), |(cost_1, cost_2), (_, pos)| {
        if visited.contains(&pos) {
            return (cost_1, cost_2);
        }
        // Euler's formula in 2D: V - E = 0
        let (area, perimeter, vertices) = flood_fill(&grid, pos, &mut visited);
        (cost_1 + area * perimeter, cost_2 + area * vertices)
    });

    println!("Part 1: {}", cost_1);
    println!("Part 2: {}", cost_2);
}
