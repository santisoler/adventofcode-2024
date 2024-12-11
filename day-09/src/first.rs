use std::cmp::Ordering;
use std::fmt;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Block {
    File { id: u32, size: u32 },
    FreeSpace { size: u32 },
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        // write!(f, "({}, {})", self.x, self.y);
        let digits = match self {
            Block::File { id, size } => id.to_string().repeat(*size as usize),
            Block::FreeSpace { size } => ".".repeat(*size as usize),
        };
        write!(f, "{}", digits)
    }
}

fn move_file_block(block: Option<Block>, disk_map: &mut Vec<Block>) -> Result<Option<Block>, bool> {
    // Print
    for i in disk_map.iter() {
        print!("{i}");
    }
    print!("\n");
    // end print
    let file_block = match block {
        Some(block) => block,
        None => return Ok(None),
    };

    let index = get_index_next_free_space(disk_map);
    let index = match index {
        Some(i) => i,
        None => return Err(true),
    };
    let space_block = &mut disk_map[index];

    match (file_block, space_block) {
        (Block::FreeSpace { size: _ }, _) => return Ok(None),
        (Block::File { id: _, size: _ }, Block::File { id: _, size: _ }) => {
            panic!()
        }
        (
            Block::File {
                id: _,
                size: file_size,
            },
            Block::FreeSpace {
                size: ref mut space_size,
            },
        ) => match file_size.cmp(&space_size) {
            Ordering::Less => {
                *space_size -= file_size;
                disk_map.insert(index, file_block);
                Ok(None)
            }
            Ordering::Greater => {
                disk_map.remove(index);
                return move_file_block(Some(file_block), disk_map);
            }
            Ordering::Equal => {
                disk_map[index] = file_block;
                Ok(None)
            }
        },
    }
}

fn get_index_next_free_space(disk_map: &Vec<Block>) -> Option<usize> {
    for (i, block) in disk_map.iter().enumerate() {
        if let Block::FreeSpace { size: _ } = block {
            return Some(i);
        }
    }
    return None;
}

fn defrag(disk_map: &mut Vec<Block>) {
    loop {
        let last_file = disk_map.pop().unwrap();
        let result = match last_file {
            Block::FreeSpace { size: _ } => continue,
            Block::File { id: _, size: _ } => move_file_block(Some(last_file), disk_map),
        };
        match result {
            Ok(_) => continue,
            Err(_) => break,
        }
    }
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

fn solve_part_one(fname: &str) -> i32 {
    let mut disk_map = read_disk_map(fname);
    defrag(&mut disk_map);
    for i in disk_map.iter() {
        print!("{i}");
    }
    print!("\n");
    0
}

fn main() {
    let fname = "data/test_input";
    let result = solve_part_one(fname);
    println!("Solution to part one: {result}");

    // let mut disk_map = vec![Block::FreeSpace { size: 2 }];
    // println!("{:?}", disk_map);
    // reduce(&mut disk_map);
    // println!("{:?}", disk_map);
}

fn reduce(disk_map: &mut Vec<Block>) {
    match disk_map[0] {
        Block::FreeSpace { ref mut size } => *size -= 1,
        Block::File { id: _, size: _ } => {}
    }
    return ();
}
