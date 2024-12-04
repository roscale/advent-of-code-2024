#![allow(non_snake_case)]

const XMAS_PATTERNS: [[(isize, isize); 3]; 8] = [
    [(1, 0), (2, 0), (3, 0)], [(-1, 0), (-2, 0), (-3, 0)],
    [(0, 1), (0, 2), (0, 3)], [(0, -1), (0, -2), (0, -3)],
    [(1, 1), (2, 2), (3, 3)], [(-1, -1), (-2, -2), (-3, -3)],
    [(1, -1), (2, -2), (3, -3)], [(-1, 1), (-2, 2), (-3, 3)],
];

const MAS_PATTERN: [[(isize, isize); 4]; 4] = [
    [(-1, -1), (1, 1), (-1, 1), (1, -1)],
    [(-1, -1), (1, 1), (1, -1), (-1, 1)],
    [(1, 1), (-1, -1), (-1, 1), (1, -1)],
    [(1, 1), (-1, -1), (1, -1), (-1, 1)],
];

trait Grid {
    fn at(&self, x: isize, y: isize) -> Option<char>;
    fn grid_iter(&self) -> GridIterator;
}

impl Grid for Vec<Vec<char>> {
    fn at(&self, x: isize, y: isize) -> Option<char> {
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
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIterator<'a> {
    type Item = ((usize, usize), char);

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cell) = self.grid.at(self.x as isize, self.y as isize) else {
            return None;
        };

        let ret = Some(((self.x, self.y), cell));

        self.x += 1;
        if self.x >= self.grid[0].len() {
            self.x = 0;
            self.y += 1;
        }

        ret
    }
}

fn main() {
    let input = include_str!("input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars()
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let Xs = grid.grid_iter()
        .filter(|&(_, c)| c == 'X')
        .map(|((x, y), _)| (x as isize, y as isize));

    let count = Xs.fold(0, |count, (x, y)| {
        let xmas_patterns = XMAS_PATTERNS.iter().filter(|pattern| {
            let letters = pattern.map(|(dx, dy)| grid.at(x + dx, y + dy));
            matches!(letters, [Some('M'), Some('A'), Some('S')])
        }).count();

        count + xmas_patterns
    });

    println!("XMAS: {}", count);

    let As = grid.grid_iter()
        .filter(|&(_, c)| c == 'A')
        .map(|((x, y), _)| (x as isize, y as isize));

    let count = As.fold(0, |count, (x, y)| {
        let mas_pattern = MAS_PATTERN.iter().filter(|pattern| {
            let letters = pattern.map(|(dx, dy)| grid.at(x + dx, y + dy));
            matches!(letters, [Some('M'), Some('S'), Some('M'), Some('S')])
        }).count();

        count + mas_pattern
    });

    println!("X-MAS: {}", count);
}
