use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|sum| println!("{}", sum))
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

fn get_checksum(line: &str) -> u128 {
    let mut blocks = parse_line(line);
    defrag(&mut blocks);
    let mut sum = 0;
    for i in 0..blocks.len() {
        match blocks[i] {
            Block::File { id } => sum += i as u128 * id,
            Block::Empty => panic!("found empty block!"),
        };
    }
    sum
}

fn defrag(blocks: &mut Vec<Block>) {
    let mut is_fragmented = true;
    while is_fragmented {
        is_fragmented = false;
        for i in 0..blocks.len() {
            match blocks[i] {
                Block::Empty => {
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
                Block::Empty => {
                    blocks.remove(i);
                }
                _ => {
                    break;
                }
            };
        }
    }
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
                fragmented.push(Block::File { id: next_id });
            } else {
                fragmented.push(Block::Empty);
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

#[derive(Debug)]
enum Block {
    File { id: u128 },
    Empty,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_checksum_test() {
        let result = get_checksum("2333133121414131402");
        assert_eq!(result, 1928);
    }
}
