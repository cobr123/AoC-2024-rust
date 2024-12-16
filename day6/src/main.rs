use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|count| println!("{}", count))
}

fn part1() -> io::Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let count = count_path(text.trim());

    Ok(count)
}

fn count_path(text: &str) -> usize {
    let (coord, table) = parse_text(text);
    let mut visited = HashSet::new();
    visited.insert(coord);
    let mut direction = Dir::UP;
    let mut new_coord = coord;
    let mut tmp_coord = get_new_pos(&coord, direction);
    while tmp_coord.row >= 0
        && tmp_coord.col >= 0
        && table.len() > tmp_coord.row as usize
        && table[0].len() > tmp_coord.col as usize
    {
        if table[tmp_coord.row as usize][tmp_coord.col as usize] == MAP::HASH {
            direction = match direction {
                Dir::UP => Dir::RIGHT,
                Dir::RIGHT => Dir::DOWN,
                Dir::DOWN => Dir::LEFT,
                Dir::LEFT => Dir::UP,
            }
        } else {
            new_coord = tmp_coord;
            visited.insert(new_coord);
        }
        tmp_coord = get_new_pos(&new_coord, direction);
    }
    visited.len()
}

fn parse_text(text: &str) -> (Pos, Vec<Vec<MAP>>) {
    let mut table = Vec::new();
    let mut guard_coord = Pos { row: 0, col: 0 };
    let mut row = 0;
    for line in text.lines() {
        let mut col = 0;
        let mut tmp = Vec::new();
        for char in line.chars() {
            match char {
                '.' => tmp.push(MAP::DOT),
                '^' => {
                    tmp.push(MAP::DOT);
                    guard_coord = Pos { row: row, col: col };
                }
                '#' => tmp.push(MAP::HASH),
                other => panic!("unknown char {other}"),
            }
            col += 1;
        }
        table.push(tmp);
        row += 1;
    }
    (guard_coord, table)
}

fn get_new_pos(pos: &Pos, dir: Dir) -> Pos {
    match dir {
        Dir::UP => Pos {
            row: pos.row - 1,
            col: pos.col,
        },
        Dir::DOWN => Pos {
            row: pos.row + 1,
            col: pos.col,
        },
        Dir::LEFT => Pos {
            row: pos.row,
            col: pos.col - 1,
        },
        Dir::RIGHT => Pos {
            row: pos.row,
            col: pos.col + 1,
        },
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum MAP {
    DOT,
    HASH,
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    row: i32,
    col: i32,
}

#[derive(Debug, Copy, Clone)]
enum Dir {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_path_test() {
        let result = count_path(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, 41);
    }
}
