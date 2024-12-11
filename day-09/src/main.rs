use std::cmp::Ordering;
use std::fmt;
use std::fs;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let fname = "data/test_input";
        let result = solve_part_one(fname);
        assert_eq!(result, 1928);
    }
}

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: u32, size: u32 },
    FreeSpace { size: u32 },
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = match self {
            Block::File { id, size } => id.to_string().repeat(*size as usize),
            Block::FreeSpace { size } => ".".repeat(*size as usize),
        };
        write!(f, "{}", digits)
    }
}

fn move_file_block(file_block: Block, disk_map: &mut Vec<Block>) -> bool {
    let index = get_index_next_free_space(disk_map);
    let index = match index {
        Some(i) => i,
        None => {
            disk_map.push(file_block); // restore last popped element
            return false;
        }
    };
    let space_block = &mut disk_map[index];

    match (file_block, space_block) {
        (Block::FreeSpace { size: _ }, _) => return true,
        (Block::File { id: _, size: _ }, Block::File { id: _, size: _ }) => {
            panic!()
        }
        (
            Block::File {
                id,
                size: ref mut file_size,
            },
            Block::FreeSpace {
                size: ref mut space_size,
            },
        ) => match file_size.cmp(&space_size) {
            Ordering::Less => {
                *space_size -= *file_size;
                disk_map.insert(index, file_block);
            }
            Ordering::Greater => {
                let file_left = Block::File {
                    id,
                    size: *space_size,
                };
                let file_right = Block::File {
                    id,
                    size: *file_size - *space_size,
                };
                disk_map[index] = file_left;
                disk_map.push(file_right);
            }
            Ordering::Equal => {
                disk_map[index] = file_block;
            }
        },
    }
    return true;
}

fn get_index_next_free_space(disk_map: &Vec<Block>) -> Option<usize> {
    for (i, block) in disk_map.iter().enumerate() {
        if let Block::FreeSpace { size: _ } = block {
            return Some(i);
        }
    }
    return None;
}

fn read_disk_map(fname: &str) -> Vec<Block> {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let digits: Vec<u32> = content
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut disk_map = vec![];
    for (i, digit) in digits.iter().enumerate() {
        if i % 2 == 0 {
            disk_map.push(Block::File {
                id: (i / 2) as u32,
                size: *digit,
            })
        } else {
            disk_map.push(Block::FreeSpace { size: *digit })
        }
    }
    return disk_map;
}

fn defrag(disk_map: &mut Vec<Block>) {
    loop {
        let last_file = disk_map.pop().unwrap();
        let result = match last_file {
            Block::FreeSpace { size: _ } => continue,
            Block::File { id: _, size: _ } => move_file_block(last_file, disk_map),
        };
        if !result {
            return ();
        }
    }
}

fn solve_part_one(fname: &str) -> u64 {
    let mut disk_map = read_disk_map(fname);
    defrag(&mut disk_map);

    let mut result = 0;
    let mut index = 0;
    for block in disk_map {
        match block {
            Block::FreeSpace { size: _ } => panic!(),
            Block::File { id, size } => {
                if index == 0 {
                    index += size;
                    continue;
                }
                result += id as u64
                    * ((index + size - 1) as u64 * (index + size) as u64 / 2
                        - ((index - 1) * index) as u64 / 2);
                index += size;
            }
        }
    }
    result
}

fn main() {
    let fname = "data/input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");
}
