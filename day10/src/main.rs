use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|sum| println!("{}", sum))
}

fn part1() -> io::Result<usize> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let sum = get_trailheads_score_sum(text.trim());

    Ok(sum)
}

fn get_trailheads_score_sum(text: &str) -> usize {
    let (start_positions, map) = parse_text(text);
    let mut sum = 0;
    for start_position in start_positions {
        let mut acc = HashSet::new();
        count_score(start_position, &map, &mut acc);
        sum += acc.len();
    }
    sum
}

fn count_score(start: Pos, map: &Vec<Vec<u8>>, acc: &mut HashSet<Pos>) {
    let current = map[start.row][start.col];
    if current == 9 {
        acc.insert(start);
    } else {
        if start.row > 0 && map[start.row - 1][start.col] == current + 1 {
            count_score(
                Pos {
                    row: start.row - 1,
                    col: start.col,
                },
                map,
                acc,
            );
        };
        if start.row + 1 < map.len() && map[start.row + 1][start.col] == current + 1 {
            count_score(
                Pos {
                    row: start.row + 1,
                    col: start.col,
                },
                map,
                acc,
            );
        };
        if start.col > 0 && map[start.row][start.col - 1] == current + 1 {
            count_score(
                Pos {
                    row: start.row,
                    col: start.col - 1,
                },
                map,
                acc,
            );
        };
        if start.col + 1 < map[start.row].len() && map[start.row][start.col + 1] == current + 1 {
            count_score(
                Pos {
                    row: start.row,
                    col: start.col + 1,
                },
                map,
                acc,
            );
        };
    }
}

fn parse_text(text: &str) -> (Vec<Pos>, Vec<Vec<u8>>) {
    let mut start_positions = Vec::new();
    let mut map = Vec::new();
    for line in text.lines() {
        let mut cols = Vec::new();
        for char in line.chars() {
            if char != '.' {
                cols.push(char.to_string().parse::<u8>().unwrap());
                if char == '0' {
                    start_positions.push(Pos {
                        row: map.len(),
                        col: cols.len() - 1,
                    });
                }
            } else {
                cols.push(0);
            }
        }
        map.push(cols);
    }
    // println!("{:?}\n{:?}", start_positions, map);
    (start_positions, map)
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct Pos {
    row: usize,
    col: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_trailheads_score_sum2_test() {
        let result = get_trailheads_score_sum(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );
        assert_eq!(result, 2);
    }

    #[test]
    fn get_trailheads_score_sum3_test() {
        let result = get_trailheads_score_sum(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn get_trailheads_score_sum4_test() {
        let result = get_trailheads_score_sum(
            "..90..9
...1.98
...2..7
6543456
765.987
876....
987....",
        );
        assert_eq!(result, 4);
    }

    #[test]
    fn get_trailheads_score_sum36_test() {
        let result = get_trailheads_score_sum(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, 36);
    }
}
