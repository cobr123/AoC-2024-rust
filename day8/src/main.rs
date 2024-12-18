use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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
    let count = count_antinodes(text.trim());

    Ok(count)
}

fn count_antinodes(text: &str) -> usize {
    let (width, height, map) = parse_text(text);
    let mut antinodes = HashSet::new();
    for list in map {
        for pair in list.iter().combinations(2) {
            let p0 = pair.get(0).unwrap();
            let p1 = pair.get(1).unwrap();
            for x in get_antinode(width, height, p0, p1) {
                antinodes.insert(x);
            }
        }
    }
    antinodes.len()
}

fn get_antinode(width: i32, height: i32, p0: &Pos, p1: &Pos) -> Vec<Pos> {
    let ((row0, col0), (row1, col1)) = if p0.row < p1.row {
        let diff_row = p1.row - p0.row;
        if p0.col < p1.col {
            let diff_col = p1.col - p0.col;
            (
                (p0.row - diff_row, p0.col - diff_col),
                (p1.row + diff_row, p1.col + diff_col),
            )
        } else {
            let diff_col = p0.col - p1.col;
            (
                (p0.row - diff_row, p0.col + diff_col),
                (p1.row + diff_row, p1.col - diff_col),
            )
        }
    } else {
        let diff_row = p0.row - p1.row;
        if p0.col < p1.col {
            let diff_col = p1.col - p0.col;
            (
                (p0.row + diff_row, p0.col - diff_col),
                (p1.row - diff_row, p1.col + diff_col),
            )
        } else {
            let diff_col = p0.col - p1.col;
            (
                (p0.row + diff_row, p0.col + diff_col),
                (p1.row - diff_row, p1.col - diff_col),
            )
        }
    };
    let mut res = Vec::new();
    if col0 >= 0 && col0 < width && row0 >= 0 && row0 < height {
        res.push(Pos {
            row: row0,
            col: col0,
        });
    }
    if col1 >= 0 && col1 < width && row1 >= 0 && row1 < height {
        res.push(Pos {
            row: row1,
            col: col1,
        });
    }
    res
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    row: i32,
    col: i32,
}

fn parse_text(text: &str) -> (i32, i32, Vec<Vec<Pos>>) {
    let mut map: HashMap<char, Vec<Pos>> = HashMap::new();
    let mut row = 0;
    let mut col = 0;
    for line in text.lines() {
        col = 0;
        for char in line.chars() {
            if char != '.' {
                match map.get(&char) {
                    Some(list) => {
                        let mut new_list: Vec<Pos> = vec![Pos { row, col }];
                        new_list.extend(list);
                        map.insert(char, new_list)
                    }
                    None => map.insert(char, vec![Pos { row, col }]),
                };
            }
            col += 1;
        }
        row += 1;
    }
    let positions = map
        .into_iter()
        .filter(|(_char, list)| list.len() > 1)
        .map(|(_char, list)| list)
        .collect();
    (row, col, positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_antinodes14_test() {
        let result = count_antinodes(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, 14);
    }

    #[test]
    fn count_antinodes2_test() {
        let result = count_antinodes(
            "..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn count_antinodes3_test() {
        let result = count_antinodes(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
......A...
..........
..........",
        );
        assert_eq!(result, 4);
    }
}
