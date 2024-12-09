use std::iter::repeat;

#[derive(Debug)]
struct Block {
    size: usize,
    id: Option<i64>,
}

fn main() {
    let input = include_str!("input.txt");

    let disk_map = input.chars().map(|x| x.to_digit(10).unwrap() as usize).collect::<Vec<_>>();

    let filesystem_size = disk_map.iter().sum();
    let mut filesystem = Vec::<i64>::with_capacity(filesystem_size);

    let mut is_file = true;
    let mut next_id = 0;

    for &n in &disk_map {
        filesystem.extend(repeat(if is_file {
            let id = next_id;
            next_id += 1;
            id
        } else {
            -1
        }).take(n));

        is_file = !is_file;
    }

    let mut i = 0;
    let mut j = filesystem.len() - 1;

    while i < j {
        if filesystem[i] != -1 {
            i += 1;
            continue;
        }
        if filesystem[j] == -1 {
            j -= 1;
            continue;
        }
        filesystem.swap(i, j);
    }

    let mut checksum = 0;
    for (i, &id) in filesystem.iter().enumerate() {
        if id == -1 {
            break;
        }
        checksum += i as i64 * id;
    }

    println!("Part 1: {}", checksum);

    let mut disk_map = disk_map.iter().scan((0, true), |(id, is_file), &n| {
        let ret = Block {
            size: n,
            id: if *is_file { Some(*id) } else { None },
        };
        if *is_file {
            *id += 1;
        }
        *is_file = !*is_file;
        Some(ret)
    }).collect::<Vec<_>>();

    for id in (0..next_id).rev() {
        let file_index = disk_map.iter().rposition(|block| {
            matches!(block, Block { id: Some(block_id), .. } if *block_id == id)
        }).unwrap();

        let free_space_index = disk_map.iter().position(|block| {
            matches!(block, Block { id: block_id, size } if block_id.is_none() && *size >= disk_map[file_index].size)
        });

        if let Some(free_space_index) = free_space_index {
            if free_space_index > file_index {
                continue;
            }

            disk_map[file_index].id = None;
            disk_map[free_space_index].size -= disk_map[file_index].size;
            disk_map.insert(free_space_index, Block { size: disk_map[file_index].size, id: Some(id) });
        }
    }

    let mut checksum = 0;
    let mut pos = 0;

    for block in disk_map.iter() {
        if let Some(id) = block.id {
            for i in pos..pos + block.size {
                checksum += i as i64 * id;
            }
        }
        pos += block.size;
    }

    println!("Part 2: {}", checksum);
}
