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

type Shortcut = ((isize, isize), (isize, isize));

fn browse_previouses(
    end: (isize, isize),
    start: (isize, isize),
    previouses: &HashMap<(isize, isize), HashSet<(isize, isize)>>,
    shortcuts: &mut HashSet<Shortcut>,
) {
    if let Some(prev) = previouses.get(&start) {
        for &previous in prev.iter() {
            browse_previouses(end, previous, previouses, shortcuts);
        }
    } else {
        shortcuts.insert((start, end));
    }
}

fn dijkstra_continue(
    grid: &Vec<Vec<char>>,
    start: (isize, isize),
    end: (isize, isize),
    queue: &mut Vec<(isize, isize)>,
    costs: &mut HashMap<(isize, isize), isize>,
    max_shortcut_reach: isize,
    shortcuts_already_taken: &HashSet<Shortcut>,
) -> HashSet<Shortcut> {
    let mut previouses: HashMap<(isize, isize), (isize, isize)> = HashMap::new();
    let mut shortcuts: HashSet<((isize, isize), (isize, isize))> = shortcuts_already_taken.clone();
    let mut shortcuts_reach = HashMap::new();
    shortcuts_reach.insert(start, max_shortcut_reach);

    while let Some(pos) = queue.pop() {
        let cost = costs[&pos];
        let reach = shortcuts_reach[&pos];

        for next_direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);
            let next_cost = cost + 1;

            let next_reach = match grid.at(next) {
                None => continue,
                Some('#') => reach - 1,
                _ => reach,
            };

            if next_cost >= *costs.get(&next).unwrap_or(&isize::MAX) {
                continue;
            }
            if next_reach < 0 {
                continue;
            }

            previouses.insert(next, pos);

            // if grid.at(pos) == Some('#') {
            //     // shortcut_previouses.insert(next, pos);
            // 
            //     if grid.at(next) != Some('#') {
            //         // let mut sh = HashSet::new();
            //         // browse_previouses(next, next, &previouses, &mut sh);
            //         // shortcuts.extend(sh);
            // 
            //         let mut current = next;
            //         while let Some(&previous) = previouses.get(&current) {
            //             // shortcuts.insert((previous, current));
            //             current = previous;
            //         }
            // 
            //         if shortcuts_already_taken.contains(&(current, next)) {
            //             continue;
            //         } else {
            //             shortcuts.insert((current, next));
            //         }
            //     }
            // }

            queue.push(next);
            costs.insert(next, next_cost);
            shortcuts_reach.insert(next, next_reach);

            queue.sort_by_key(|&pos| costs[&pos]);

            // if let Some(&n) = shortcuts_reach.get(&next) {
            //     if n <= next_reach {
            //         shortcuts_reach.insert(next, next_reach);
            //     }
            // } else {
            //     shortcuts_reach.insert(next, next_reach);
            // }
        }
    }

    shortcuts
}

// fn dijkstra(grid: &Vec<Vec<char>>, start: (isize, isize), end: (isize, isize)) -> HashMap<isize, HashSet<(isize, isize)>> {
//     let mut shortcuts: HashMap<isize, HashSet<_>> = HashMap::new();
//
//     let mut queue = Vec::new();
//     let mut costs = HashMap::new();
//
//     costs.insert(start, 0);
//     queue.push(start);
//
//     let (base_distance, potential_shortcuts) = dijkstra_find_potential_shortcuts(grid, end, &mut queue, &mut costs);
//     assert!(queue.is_empty());
//
//     potential_shortcuts.iter().for_each(|&potential_shortcut| {
//         // queue.push(start);
//         let distance = dijkstra_with_shortcut(grid, start, end, potential_shortcut);
//         // assert!(queue.is_empty());
//
//         shortcuts.entry(base_distance - distance).or_default().insert(potential_shortcut);
//     });
//
//     shortcuts
// }

// fn dijkstra_with_shortcut(
//     grid: &Vec<Vec<char>>,
//     start: (isize, isize),
//     end: (isize, isize),
//     shortcut: (isize, isize),
// ) -> isize {
//     let mut queue = Vec::new();
//     let mut costs = HashMap::new();
//
//     costs.insert(start, 0);
//     queue.push(start);
//
//     while let Some(pos) = queue.pop() {
//         let cost = costs[&pos];
//
//         for next_direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
//             let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);
//             let next_cost = cost + 1;
//
//             if !matches!(grid.at(next), Some('.' | 'E')) && next != shortcut {
//                 continue;
//             }
//
//             if next_cost < *costs.entry(next).or_insert(isize::MAX) {
//                 queue.push(next);
//                 costs.insert(next, next_cost);
//             }
//         }
//     }
//     costs[&end]
// }

// fn dijkstra_find_potential_shortcuts(
//     grid: &Vec<Vec<char>>,
//     end: (isize, isize),
//     queue: &mut Vec<(isize, isize)>,
//     costs: &mut HashMap<(isize, isize), isize>,
// ) -> (isize, Vec<(isize, isize)>) {
//     let mut potential_shortcuts = Vec::new();
//
//     while let Some(pos) = queue.pop() {
//         let cost = costs[&pos];
//
//         for next_direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
//             let next = (pos.0 + next_direction.0, pos.1 + next_direction.1);
//
//             if matches!(grid.at(next), Some('#')) {
//                 potential_shortcuts.push(next);
//             }
//
//             if !matches!(grid.at(next), Some('.' | 'E')) {
//                 continue;
//             }
//
//             let next_cost = cost + 1;
//             if next_cost < *costs.entry(next).or_insert(isize::MAX) {
//                 queue.push(next);
//                 costs.insert(next, next_cost);
//             }
//         }
//     }
//     (costs[&end], potential_shortcuts)
// }

fn main() {
    let input = include_str!("input.txt");

    let grid = input.lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start = grid.grid_iter().find(|&(cell, _)| cell == 'S').unwrap().1;
    let end = grid.grid_iter().find(|&(cell, _)| cell == 'E').unwrap().1;

    // let base_path = dijkstra_continue(&grid, end, &mut queue, &mut costs, 0);
    // dbg!(base_path);

    // {
    //     let mut queue = Vec::new();
    //     let mut costs = HashMap::new();
    //
    //     costs.insert(start, 0);
    //     queue.push(start);
    //
    //     let (cost, shortcut) = dijkstra_continue(&grid, start, end, &mut queue, &mut costs, 1);
    //
    //     let mut queue = Vec::new();
    //     let mut costs = HashMap::new();
    //
    //     costs.insert(start, 0);
    //     queue.push(start);
    //     costs.insert((6, 7), isize::MIN);
    //
    //     let a = dijkstra_continue(&grid, start, end, &mut queue, &mut costs, 1);
    //     dbg!(a);
    // }

    // return;


    // let base_distance = {
    //     let mut queue = Vec::new();
    //     let mut costs = HashMap::new();
    //
    //     costs.insert(start, 0);
    //     queue.push(start);
    //
    //     let (base_distance, _) = dijkstra_continue(&grid, start, end, &mut queue, &mut costs, 0, &HashSet::new());
    //     base_distance
    // };

    // let mut shortcut_set: HashSet<Shortcut> = HashSet::new();
    // let mut shortcuts: HashMap<isize, Vec<Shortcut>> = HashMap::new();

    let mut queue = Vec::new();
    let mut costs = HashMap::new();

    costs.insert(start, 0);
    queue.push(start);

    // for &s in shortcut_set.iter() {
    //     costs.insert(s, isize::MIN);
    // }

    let shortcuts = dijkstra_continue(&grid, start, end, &mut queue, &mut costs, 1, &HashSet::new());
    let shortcuts = dijkstra_continue(&grid, start, end, &mut queue, &mut costs, 1, &shortcuts);
    println!("len {:?}", shortcuts.len());

    // for (cost, shortcut) in shortcuts.iter() {
    //     println!("{}: {:?}", cost, shortcut);
    // }
    //
    // let time_saved: usize = shortcuts.iter().filter_map(|(&time_saved, cheats)| {
    //     if time_saved >= 100 {
    //         Some(cheats.len())
    //     } else {
    //         None
    //     }
    // }).sum();
    //
    // println!("Part 1: {}", time_saved);
}
