use std::collections::HashMap;
use itertools::Itertools;

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

fn dijkstra(grid: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> Vec<Vec<(isize, isize)>> {
    if start == end {
        return vec![];
    }

    let mut queue = Vec::new();
    let mut costs = HashMap::new();
    let mut previous: HashMap<(isize, isize), Vec<(isize, isize)>> = HashMap::new();

    costs.insert(start, 0);
    queue.push(start);

    while let Some(pos) = queue.pop() {
        let cost = costs[&pos];

        for next_direction in [(1, 0), (0, -1), (0, 1), (-1, 0)] {
            let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);

            if matches!(grid.at(next), None | Some('#')) {
                continue;
            }

            let next_cost = cost + 1;

            if next_cost < *costs.entry(next).or_insert(isize::MAX)
            {
                queue.push(next);
                costs.insert(next, next_cost);
                previous.entry(next).or_default().push(pos);
            }
        }
    }

    let mut path = Vec::new();
    let mut p = end;
    path.push(p);
    while p != start {
        p = previous[&p];
        path.push(p);
    }
    path.reverse();
    path
}

fn get_directional_code(keypad: &Vec<Vec<char>>, code: &Vec<char>, directional_keypad: bool) -> Vec<char> {
    let mut directions = Vec::new();

    code.iter().fold('A', |start, &end| {
        let start_pos = keypad.grid_iter().find(|(cell, _)| *cell == start).unwrap().1;
        let end_pos = keypad.grid_iter().find(|(cell, _)| *cell == end).unwrap().1;

        let path = dijkstra(&keypad, start_pos, end_pos, directional_keypad);
        let path = path.iter().tuple_windows().map(|(from, to)| {
            (to.0 - from.0, to.1 - from.1)
        }).collect::<Vec<_>>();

        directions.push(path);

        end
    });

    let mut directional_code = Vec::new();

    for path in &directions {
        for direction in path {
            match direction {
                (1, 0) => directional_code.push('>'),
                (0, 1) => directional_code.push('v'),
                (-1, 0) => directional_code.push('<'),
                (0, -1) => directional_code.push('^'),
                _ => unreachable!(),
            }
        }
        directional_code.push('A');
    }
    println!("{}", directional_code.iter().collect::<String>());
    println!("len {}", directional_code.iter().collect::<String>().len());
    directional_code
}

fn code_complexity(code: &(&str, Vec<char>)) -> usize {
    let numeric_part = code.0.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<usize>().unwrap();

    println!("{}", code.1.iter().collect::<String>());
    dbg!(numeric_part) * dbg!(code.1.len())
}

fn main() {
    let input = include_str!("input.txt");
    let codes = input.lines();

    let numeric_keypad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];

    let directional_keypad = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>'],
    ];

    let codes = codes.map(|code| {
        let code_char_vec = code.chars().collect::<Vec<_>>();

        let (directional_code, _) = (0..3).fold((code_char_vec, &numeric_keypad), |(code, keypad), _| {
            let directional_code = get_directional_code(keypad, &code, keypad == &directional_keypad);
            (directional_code, &directional_keypad)
        });

        (code, directional_code)
    }).collect::<Vec<_>>();

    let complexity = codes.iter().map(code_complexity).sum::<usize>();

    println!("Part 1: {}", complexity);

    // let code = "<A>Av<<AA>^AA>AvAA^A<vAAA>^A";
    // let mut pos = (2, 0);
    // for c in code.chars() {
    //     pos = match c {
    //         '^' => (pos.0, pos.1 - 1),
    //         'v' => (pos.0, pos.1 + 1),
    //         '<' => (pos.0 - 1, pos.1),
    //         '>' => (pos.0 + 1, pos.1),
    //         'A' => {
    //             print!("{}", numeric_keypad.at(pos).unwrap());
    //             pos
    //         }
    //         _ => unreachable!(),
    //     };
    // }
}
