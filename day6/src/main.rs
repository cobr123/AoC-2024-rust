use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|count| println!("{}", count))?;
    part2().map(|count| println!("{}", count))
}

fn part1() -> io::Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let count = count_path(text.trim()).len();

    Ok(count)
}

fn part2() -> io::Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let count = count_path_loops(text.trim());

    Ok(count)
}

fn count_path(text: &str) -> HashSet<Pos> {
    let (coord, table) = parse_text(text);
    let mut visited = HashSet::new();
    visited.insert(coord);
    let mut direction = Dir::UP;
    let mut new_coord = coord;
    let mut tmp_coord = get_new_pos(&new_coord, direction);
    while is_on_map(tmp_coord, &table) {
        if table[tmp_coord.row as usize][tmp_coord.col as usize] == MAP::HASH {
            direction = get_direction(direction)
        } else {
            new_coord = tmp_coord;
            visited.insert(new_coord);
        }
        tmp_coord = get_new_pos(&new_coord, direction);
    }
    visited
}

fn count_path_loops(text: &str) -> usize {
    let mut path = count_path(text);
    let mut loops: HashSet<String> = HashSet::new();
    let (coord, table) = parse_text(text);
    path.remove(&coord);
    for pos in path {
        let mut tmp_table = table.clone();
        tmp_table[pos.row as usize][pos.col as usize] = MAP::HASH;
        let (new_loop, is_loop) = is_looped(coord, &tmp_table);
        if is_loop {
            loops.insert(
                new_loop
                    .iter()
                    .map(|x| format!("{},{}", x.row, x.col))
                    .collect::<Vec<String>>()
                    .join("|"),
            );
        }
    }
    loops.len()
}

fn is_looped(coord: Pos, table: &Vec<Vec<MAP>>) -> (HashSet<Pos>, bool) {
    let mut visited = HashSet::new();
    let mut visited_count: HashMap<Pos, usize> = HashMap::new();
    visited.insert(coord);
    visited_count.insert(coord, 1);
    let mut direction = Dir::UP;
    let mut new_coord = coord;
    let mut tmp_coord = get_new_pos(&new_coord, direction);
    while is_on_map(tmp_coord, &table) {
        if table[tmp_coord.row as usize][tmp_coord.col as usize] == MAP::HASH {
            direction = get_direction(direction);
        } else {
            new_coord = tmp_coord;
            visited.insert(new_coord);
            match visited_count.get(&new_coord) {
                Some(count) => {
                    if *count > 9 {
                        return (visited, true);
                    }
                    visited_count.insert(new_coord, *count + 1);
                }
                None => {
                    visited_count.insert(new_coord, 1);
                }
            };
        }
        tmp_coord = get_new_pos(&new_coord, direction);
    }
    (visited, false)
}

fn get_direction(direction: Dir) -> Dir {
    match direction {
        Dir::UP => Dir::RIGHT,
        Dir::RIGHT => Dir::DOWN,
        Dir::DOWN => Dir::LEFT,
        Dir::LEFT => Dir::UP,
    }
}

fn is_on_map(pos: Pos, table: &Vec<Vec<MAP>>) -> bool {
    pos.row >= 0
        && pos.col >= 0
        && table.len() > pos.row as usize
        && table[0].len() > pos.col as usize
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

#[derive(PartialEq, Debug, Copy, Clone)]
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
        )
        .len();
        assert_eq!(result, 41);
    }

    #[test]
    fn count_path_loops_test() {
        let result = count_path_loops(
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
        assert_eq!(result, 6);
    }

    #[test]
    fn count_path_one_way_loop_test() {
        let result = count_path_loops(
            "
..#.#.....
..........
.#..^....#
........#."
                .trim(),
        );
        assert_eq!(result, 1);
    }
}
