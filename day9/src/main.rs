use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|sum| println!("{}", sum))?;
    part2().map(|sum| println!("{}", sum))
}

fn part1() -> io::Result<u128> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let sum = get_checksum(text.trim());

    Ok(sum)
}

fn part2() -> io::Result<u128> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let sum = get_checksum2(text.trim());

    Ok(sum)
}

fn get_checksum(line: &str) -> u128 {
    let mut blocks = parse_line(line);
    defrag_by_blocks(&mut blocks);
    let mut sum = 0;
    for i in 0..blocks.len() {
        match blocks[i] {
            Block::File { id, size: _ } => sum += i as u128 * id,
            Block::Empty { size: _ } => panic!("found empty block!"),
        };
    }
    sum
}

fn get_checksum2(line: &str) -> u128 {
    let blocks = parse_line(line);
    let new_blocks = defrag_by_files(&blocks);
    let mut sum = 0;
    // println!("");
    for i in 0..new_blocks.len() {
        match new_blocks[i] {
            Block::File { id, size: _ } => {
                // print!("{}", id);
                sum += i as u128 * id;
            }
            Block::Empty { size: _ } => {
                // print!(".");
            }
        };
    }
    // println!("");
    sum
}

fn defrag_by_blocks(blocks: &mut Vec<Block>) {
    let mut is_fragmented = true;
    while is_fragmented {
        is_fragmented = false;
        for i in 0..blocks.len() {
            match blocks[i] {
                Block::Empty { size: _ } => {
                    let last_idx = blocks.len() - 1;
                    blocks.swap(i, last_idx);
                    blocks.remove(last_idx);
                    is_fragmented = true;
                    break;
                }
                _ => (),
            };
        }
        for i in (0..blocks.len()).rev() {
            match blocks[i] {
                Block::Empty { size: _ } => {
                    blocks.remove(i);
                }
                _ => {
                    break;
                }
            };
        }
    }
}

fn get_reversed_file_indexes(blocks: &Vec<Block>) -> Vec<usize> {
    let mut files_map: HashMap<u128, (&Block, usize)> = HashMap::new();
    blocks
        .iter()
        .enumerate()
        .for_each(|(idx, block)| match block {
            Block::File { id, size: _ } => {
                if !files_map.contains_key(id) {
                    files_map.insert(*id, (block, idx));
                }
            }
            _ => {}
        });
    let mut files: Vec<usize> = files_map.values().map(|(_, idx)| *idx).collect();
    files.sort_by(|idx1, idx2| idx2.cmp(idx1));
    files
}

fn defrag_by_files(blocks: &Vec<Block>) -> Vec<Block> {
    let file_indexes = get_reversed_file_indexes(blocks);
    let mut new_blocks = blocks.clone();
    // println!("{:?}", files);
    for file_idx in file_indexes {
        match blocks.get(file_idx).unwrap() {
            Block::File {
                id: _,
                size: file_size,
            } => {
                for i in 0..file_idx {
                    match new_blocks[i] {
                        Block::Empty { size: empty_size } if empty_size >= *file_size => {
                            for k in 0..*file_size as usize {
                                // println!(
                                //     "swap {} {} {:?} {:?}",
                                //     i + k,
                                //     file_idx + k,
                                //     new_blocks.get(i + k).unwrap(),
                                //     new_blocks.get(file_idx + k).unwrap()
                                // );
                                new_blocks.swap(i + k, file_idx + k);
                            }
                            if empty_size > *file_size {
                                for k in *file_size..empty_size {
                                    // println!(
                                    //     "new Empty size {} pos {}\n",
                                    //     empty_size - *file_size,
                                    //     i + k as usize
                                    // );
                                    new_blocks[i + k as usize] = Block::Empty {
                                        size: empty_size - *file_size,
                                    };
                                }
                            }
                            break;
                        }
                        _ => (),
                    };
                }
            }
            _ => {}
        };
    }
    new_blocks
}

fn parse_line(line: &str) -> Vec<Block> {
    let mut fragmented = Vec::new();
    let mut next_id = 0;
    let mut is_file = true;
    for char in line.chars() {
        let block_size = match char {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            other => panic!("unknown char {other}"),
        };
        for _ in 0..block_size {
            if is_file {
                fragmented.push(Block::File {
                    id: next_id,
                    size: block_size,
                });
            } else {
                fragmented.push(Block::Empty { size: block_size });
            }
        }
        if is_file {
            next_id += 1;
        }
        is_file = !is_file;
    }
    // println!("last block is {:?}", fragmented[fragmented.len()-1]);
    fragmented
}

#[derive(Debug, Clone)]
enum Block {
    File { id: u128, size: u128 },
    Empty { size: u128 },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_checksum_test() {
        let result = get_checksum("2333133121414131402");
        assert_eq!(result, 1928);
    }

    #[test]
    fn get_checksum2_test() {
        let result = get_checksum2("2333133121414131402");
        assert_eq!(result, 2858);
    }
}
