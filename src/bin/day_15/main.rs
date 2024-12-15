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

fn get_direction(movement: char) -> (isize, isize) {
    match movement {
        '<' => (-1, 0),
        '>' => (1, 0),
        '^' => (0, -1),
        'v' => (0, 1),
        _ => unreachable!()
    }
}

fn in_direction(grid: &Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) -> impl Iterator<Item=(isize, isize)> + '_ {
    (0..).scan(pos, move |pos, _| {
        let next = (pos.0 + direction.0, pos.1 + direction.1);
        if grid.at(next).is_some() {
            *pos = next;
            Some(next)
        } else {
            None
        }
    })
}

fn empty_space_in_direction(grid: &Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) -> Option<(isize, isize)> {
    let in_direction = in_direction(grid, pos, direction);
    let mut until_wall = in_direction.take_while(|pos| grid.at(*pos) != Some('#'));
    until_wall.find(|pos| grid.at(*pos) == Some('.'))
}

fn can_push(grid: &Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) -> bool {
    if matches!(direction, (-1, 0) | (1, 0)) {
        return empty_space_in_direction(grid, pos, direction).is_some();
    }

    let in_front = (pos.0 + direction.0, pos.1 + direction.1);
    match grid.at(in_front) {
        Some('.') => true,
        Some('O') => empty_space_in_direction(grid, pos, direction).is_some(),
        Some('#') => false,
        Some('[') => {
            let right_side = (in_front.0 + 1, in_front.1);
            can_push(grid, in_front, direction) && can_push(grid, right_side, direction)
        }
        Some(']') => {
            let left_side = (in_front.0 - 1, in_front.1);
            can_push(grid, in_front, direction) && can_push(grid, left_side, direction)
        }
        _ => unreachable!()
    }
}

fn push(grid: &mut Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) {
    fn shift(grid: &mut Vec<Vec<char>>, pos: (isize, isize), direction: (isize, isize)) {
        let empty_space = empty_space_in_direction(grid, pos, direction).unwrap();
        let mut cell = empty_space;
        while cell != pos {
            let prev = (cell.0 - direction.0, cell.1 - direction.1);
            grid[cell.1 as usize][cell.0 as usize] = grid[prev.1 as usize][prev.0 as usize];
            cell = prev;
        }
        grid[pos.1 as usize][pos.0 as usize] = '.';
    }

    if matches!(direction, (-1, 0) | (1, 0)) {
        shift(grid, pos, direction);
        return;
    }

    let in_front = (pos.0 + direction.0, pos.1 + direction.1);
    match grid.at(in_front) {
        Some('.') => {}
        Some('O') => shift(grid, pos, direction),
        Some('[') => {
            let right_side = (in_front.0 + 1, in_front.1);
            push(grid, in_front, direction);
            push(grid, right_side, direction);
        }
        Some(']') => {
            let left_side = (in_front.0 - 1, in_front.1);
            push(grid, in_front, direction);
            push(grid, left_side, direction);
        }
        _ => unreachable!()
    }

    grid[in_front.1 as usize][in_front.0 as usize] = grid[pos.1 as usize][pos.0 as usize];
    grid[pos.1 as usize][pos.0 as usize] = '.'
}

fn run(grid: &mut Vec<Vec<char>>, movements: &Vec<char>) {
    let mut robot = grid.grid_iter().find(|(cell, _)| *cell == '@').unwrap().1;

    for movement in movements.iter().copied() {
        let direction = get_direction(movement);
        let in_front = (robot.0 + direction.0, robot.1 + direction.1);

        if can_push(&grid, robot, direction) {
            push(grid, robot, direction);
            robot = in_front;
        }
    }
}

fn gps_sum(grid: &Vec<Vec<char>>, char: char) -> isize {
    grid.grid_iter()
        .filter(|(cell, _)| *cell == char)
        .map(|(_, (x, y))| 100 * y + x)
        .sum::<isize>()
}

fn main() {
    let input = include_str!("input.txt");

    let (grid, movements) = input.split_once("\n\n").unwrap();

    let starting_grid = grid.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let movements = movements.lines()
        .map(|line| line.chars())
        .flatten()
        .collect::<Vec<_>>();

    let mut grid = starting_grid.clone();
    run(&mut grid, &movements);
    println!("Part 1: {}", gps_sum(&grid, 'O'));

    let mut grid = starting_grid.into_iter()
        .map(|row| row.into_iter()
            .flat_map(|cell| {
                match cell {
                    '#' => "##",
                    'O' => "[]",
                    '.' => "..",
                    '@' => "@.",
                    _ => unreachable!()
                }.chars()
            })
            .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    run(&mut grid, &movements);
    println!("Part 2: {}", gps_sum(&grid, '['));
}
