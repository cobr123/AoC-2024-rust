use itertools::Itertools;
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
    let count = count_antinodes(text.trim());

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
    let count = count_antinodes_with_harmonics(text.trim());

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

fn count_antinodes_with_harmonics(text: &str) -> usize {
    let (width, height, map) = parse_text(text);
    let mut antinodes = HashSet::new();
    for list in map {
        for pair in list.iter().combinations(2) {
            let p0 = pair.get(0).unwrap();
            let p1 = pair.get(1).unwrap();
            get_antinode_from_left(&mut antinodes, width, height, p0, p1);
            get_antinode_from_right(&mut antinodes, width, height, p0, p1);
        }
    }
    // print_antinodes(width, height, &antinodes);
    antinodes.len()
}

fn print_antinodes(width: i32, height: i32, antinodes: &HashSet<Pos>) {
    for r in 0..height {
        for c in 0..width {
            if antinodes.iter().contains(&Pos { row: r, col: c }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    // antinodes.iter().for_each(|x| println!("{:?}", x));
}

fn get_antinode_from_left(
    antinodes: &mut HashSet<Pos>,
    width: i32,
    height: i32,
    p0: &Pos,
    p1: &Pos,
) {
    antinodes.insert(*p0);
    antinodes.insert(*p1);
    // print_antinodes(width, height, &antinodes);
    if p0.col == p1.col {
        if p0.row < p1.row {
            let diff_row = p1.row - p0.row;
            let (row, col) = (p0.row - diff_row, p0.col);
            if col >= 0 && col < width && row >= 0 && row < height {
                let pp = Pos { row, col };
                get_antinode_from_left(antinodes, width, height, &pp, p1);
            }
        } else if p1.row < p0.row {
            let diff_row = p0.row - p1.row;
            let (row, col) = (p1.row - diff_row, p0.col);
            if col >= 0 && col < width && row >= 0 && row < height {
                let pp = Pos { row, col };
                get_antinode_from_left(antinodes, width, height, &pp, p1);
            }
        }
    } else if p0.col < p1.col {
        let diff_col = p1.col - p0.col;
        let (row, col) = if p0.row < p1.row {
            let diff_row = p1.row - p0.row;
            (p0.row - diff_row, p0.col - diff_col)
        } else {
            let diff_row = p0.row - p1.row;
            (p0.row + diff_row, p0.col - diff_col)
        };
        if col >= 0 && col < width && row >= 0 && row < height {
            let pp1 = Pos { row, col };
            // println!("l {:?} {:?}, {:?} {:?}", pp1, pp0, p0, p1);
            get_antinode_from_left(antinodes, width, height, &pp1, p0);
        } else {
            // println!(
            //     "else l {:?} height={height}, width={width}",
            //     Pos { row, col }
            // )
        }
    } else {
        // println!("swap l {:?} {:?}", p0, p1);
        get_antinode_from_left(antinodes, width, height, p1, p0);
    }
}

fn get_antinode_from_right(
    antinodes: &mut HashSet<Pos>,
    width: i32,
    height: i32,
    p0: &Pos,
    p1: &Pos,
) {
    antinodes.insert(*p0);
    antinodes.insert(*p1);
    // print_antinodes(width, height, &antinodes);
    if p0.col == p1.col {
        if p1.row > p0.row {
            let diff_row = p1.row - p0.row;
            let (row, col) = (p1.row + diff_row, p0.col);
            if col >= 0 && col < width && row >= 0 && row < height {
                let pp = Pos { row, col };
                get_antinode_from_right(antinodes, width, height, &pp, p0);
            }
        } else if p0.row > p1.row {
            let diff_row = p0.row - p1.row;
            let (row, col) = (p0.row + diff_row, p0.col);
            if col >= 0 && col < width && row >= 0 && row < height {
                let pp = Pos { row, col };
                get_antinode_from_right(antinodes, width, height, &pp, p0);
            }
        }
    } else if p0.col > p1.col {
        let diff_col = p0.col - p1.col;
        let (row, col) = if p0.row < p1.row {
            let diff_row = p1.row - p0.row;
            (p0.row - diff_row, p0.col + diff_col)
        } else {
            let diff_row = p0.row - p1.row;
            (p0.row + diff_row, p0.col + diff_col)
        };
        if col >= 0 && col < width && row >= 0 && row < height {
            let pp1 = Pos { row, col };
            // println!("r {:?} {:?}, {:?} {:?}", pp1, pp0, p0, p1);
            get_antinode_from_right(antinodes, width, height, p0, &pp1);
        } else {
            // println!(
            //     "else r {:?} height={height}, width={width}",
            //     Pos { row, col }
            // )
        }
    } else {
        // println!("swap r {:?} {:?}", p0, p1);
        get_antinode_from_right(antinodes, width, height, p1, p0);
    }
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
    (col, row, positions)
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

    #[test]
    fn count_antinodes_with_harmonics9_test() {
        let result = crate::count_antinodes_with_harmonics(
            "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........",
        );
        assert_eq!(result, 9);
    }

    #[test]
    fn count_antinodes_with_harmonics34_test() {
        let result = crate::count_antinodes_with_harmonics(
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
        assert_eq!(result, 34);
    }

    #[test]
    fn count_antinodes_with_harmonics_vt_test() {
        let result = crate::count_antinodes_with_harmonics(
            "......
......
.....0
......
.....0",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_vb_test() {
        let result = crate::count_antinodes_with_harmonics(
            "......
.....0
......
.....0
......
......",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_btl_test() {
        let result = crate::count_antinodes_with_harmonics(
            "......
....0.
.....0",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_btl2_test() {
        let result = crate::count_antinodes_with_harmonics(
            "............
............
............
............
............
............
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, 12);
    }

    #[test]
    fn count_antinodes_with_harmonics_btr_test() {
        let result = crate::count_antinodes_with_harmonics(
            "......
....0.
...0..",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_tbl_test() {
        let result = crate::count_antinodes_with_harmonics(
            ".....0
....0.
......",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_tbl2_test() {
        let result = crate::count_antinodes_with_harmonics(
            "............
........0...
............
.......0....
............
............
............
............
............
............
............
............",
        );
        assert_eq!(result, 6);
    }

    #[test]
    fn count_antinodes_with_harmonics_tbr_test() {
        let result = crate::count_antinodes_with_harmonics(
            "...0..
....0.
......",
        );
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_hr_test() {
        let result = crate::count_antinodes_with_harmonics("0.0..");
        assert_eq!(result, 3);
    }

    #[test]
    fn count_antinodes_with_harmonics_hl_test() {
        let result = crate::count_antinodes_with_harmonics("...0.0");
        assert_eq!(result, 3);
    }
}
