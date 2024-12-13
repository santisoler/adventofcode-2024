use std::cmp::Ordering;
use std::fmt;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Blocks {
    Free(u32),
    File(u32, u32),
}

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let digits = match self {
            Blocks::File(id, size) => id.to_string().repeat(*size as usize),
            Blocks::Free(size) => ".".repeat(*size as usize),
        };
        write!(f, "{}", digits)
    }
}

struct Disk {
    disk: Vec<Blocks>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self.disk.iter().map(|x| format!("{x}")).collect();
        write!(f, "{}", s)
    }
}

impl Disk {
    pub fn insert(&mut self, file_index: usize, space_index: usize) -> Result<bool, String> {
        // Insert a file in a given free space
        //
        // Returns:
        //   Ok(true): the file was moved to the free space.
        //   Ok(false): the file cannot fit the available space.
        //   Err(_): the file index doesn't match a file or the space
        //           index doesn't match a space.

        let file = self.disk[file_index];
        let space = &mut self.disk[space_index];
        match (file, space) {
            (_, Blocks::File(_, _)) => return Err("Cannot insert block in file.".to_string()),
            (Blocks::Free(_), _) => return Err("Cannot insert space in block.".to_string()),
            (Blocks::File(_, file_size), Blocks::Free(ref mut space_size)) => {
                match file_size.cmp(space_size) {
                    Ordering::Less => {
                        let diff = *space_size - file_size;
                        *space_size = file_size;
                        self.disk.swap(file_index, space_index);
                        self.disk.insert(space_index + 1, Blocks::Free(diff));
                        self.merge_free_spaces();
                    }
                    Ordering::Equal => {
                        self.disk.swap(file_index, space_index);
                        // merge free spaces. use the file_index because now that
                        // is where the space is leaving after the swap.
                        self.merge_free_spaces();
                    }
                    Ordering::Greater => return Ok(false),
                }
            }
        };
        Ok(true)
    }

    pub fn checksum(&self) -> u64 {
        let mut checksum = 0;
        let mut index = 0;
        for block in self.disk.iter() {
            match block {
                Blocks::Free(size) => index += size,
                Blocks::File(id, size) => {
                    if index == 0 {
                        index += size;
                        continue;
                    }
                    checksum += *id as u64
                        * ((index + size - 1) as u64 * (index + size) as u64 / 2
                            - ((index - 1) * index) as u64 / 2);
                    index += size;
                }
            }
        }
        checksum
    }

    fn merge_free_spaces(&mut self) {
        let mut index = 0;
        while index < self.disk.len() - 1 {
            if let Blocks::File(_, _) = &mut self.disk[index] {
                index += 1;
                continue;
            }
            let next_size = self.get_free_space_size(index + 1);
            if let Some(next_size) = next_size {
                if let Blocks::Free(ref mut size) = &mut self.disk[index] {
                    *size += next_size;
                    self.disk.remove(index + 1);
                    continue;
                }
            }
            index += 1
        }
    }

    fn get_free_space_size(&self, index: usize) -> Option<u32> {
        if index >= self.disk.len() - 1 {
            return None;
        }
        match self.disk[index] {
            Blocks::File(_, _) => None,
            Blocks::Free(size) => Some(size),
        }
    }
}

fn move_file<'a>(block_index: usize, disk: &mut Disk) -> bool {
    if let Blocks::Free(_) = disk.disk[block_index] {
        return false; // if block is a free space, we cannot move it, so return false
    };

    let mut free_space_index = 0;
    while free_space_index < block_index {
        if let Blocks::File(_, _) = disk.disk[free_space_index] {
            free_space_index += 1;
            continue;
        };
        let result = disk.insert(block_index, free_space_index);
        match result {
            Ok(code) => match code {
                true => return true,
                false => {
                    free_space_index += 1;
                    continue;
                }
            },
            Err(txt) => panic!("{}", txt),
        }
    }
    return false;
}

fn defrag(disk: &mut Disk) {
    // Defrag disk by moving entire files
    let mut rev_index = 0; // reverse index (index counting from the right)
    loop {
        let index = disk.disk.len() - 1 - rev_index;
        if index == 0 {
            break;
        };
        if !move_file(index, disk) {
            rev_index += 1;
        };
    }
}

fn read_disk_map(fname: &str) -> Disk {
    let content = fs::read_to_string(fname).expect("Couldn't read");
    let digits: Vec<u32> = content
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut disk = vec![];
    for (i, digit) in digits.iter().enumerate() {
        if i % 2 == 0 {
            disk.push(Blocks::File((i / 2) as u32, *digit))
        } else {
            disk.push(Blocks::Free(*digit))
        }
    }
    return Disk { disk };
}

pub fn solve_part_two(fname: &str) -> u64 {
    let mut disk = read_disk_map(fname);
    defrag(&mut disk);
    return disk.checksum();
}
