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
    let count = count_xmas(text.trim());

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
    let count = count_mas(text.trim());

    Ok(count)
}

#[derive(Debug, Copy, Clone)]
enum XMAS {
    X,
    M,
    A,
    S,
}

fn count_xmas(text: &str) -> usize {
    let table = parse_text(text);
    count_horizontal(&table)
        + count_horizontal_rev(&table)
        + count_vertical(&table)
        + count_vertical_rev(&table)
        + count_diagonal_l_to_r(&table)
        + count_diagonal_l_to_r_rev(&table)
        + count_diagonal_r_to_l(&table)
        + count_diagonal_r_to_l_rev(&table)
}

fn count_mas(text: &str) -> usize {
    let table = parse_text(text);
    count_mas_top(&table)
        + count_mas_right(&table)
        + count_mas_bottom(&table)
        + count_mas_left(&table)
}

fn count_mas_top(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 2 {
        for col in 0..table[0].len() - 2 {
            match (
                table[row][col],
                table[row][col + 2],
                table[row + 1][col + 1],
                table[row + 2][col],
                table[row + 2][col + 2],
            ) {
                (XMAS::M, XMAS::M, XMAS::A, XMAS::S, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_mas_bottom(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 2 {
        for col in 0..table[0].len() - 2 {
            match (
                table[row][col],
                table[row][col + 2],
                table[row + 1][col + 1],
                table[row + 2][col],
                table[row + 2][col + 2],
            ) {
                (XMAS::S, XMAS::S, XMAS::A, XMAS::M, XMAS::M) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_mas_right(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 2 {
        for col in 0..table[0].len() - 2 {
            match (
                table[row][col],
                table[row][col + 2],
                table[row + 1][col + 1],
                table[row + 2][col],
                table[row + 2][col + 2],
            ) {
                (XMAS::S, XMAS::M, XMAS::A, XMAS::S, XMAS::M) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_mas_left(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 2 {
        for col in 0..table[0].len() - 2 {
            match (
                table[row][col],
                table[row][col + 2],
                table[row + 1][col + 1],
                table[row + 2][col],
                table[row + 2][col + 2],
            ) {
                (XMAS::M, XMAS::S, XMAS::A, XMAS::M, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_horizontal(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col],
                table[row][col + 1],
                table[row][col + 2],
                table[row][col + 3],
            ) {
                (XMAS::X, XMAS::M, XMAS::A, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_horizontal_rev(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col],
                table[row][col + 1],
                table[row][col + 2],
                table[row][col + 3],
            ) {
                (XMAS::S, XMAS::A, XMAS::M, XMAS::X) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_vertical(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() {
            match (
                table[row][col],
                table[row + 1][col],
                table[row + 2][col],
                table[row + 3][col],
            ) {
                (XMAS::X, XMAS::M, XMAS::A, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_vertical_rev(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() {
            match (
                table[row][col],
                table[row + 1][col],
                table[row + 2][col],
                table[row + 3][col],
            ) {
                (XMAS::S, XMAS::A, XMAS::M, XMAS::X) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_diagonal_l_to_r(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col],
                table[row + 1][col + 1],
                table[row + 2][col + 2],
                table[row + 3][col + 3],
            ) {
                (XMAS::X, XMAS::M, XMAS::A, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_diagonal_l_to_r_rev(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col],
                table[row + 1][col + 1],
                table[row + 2][col + 2],
                table[row + 3][col + 3],
            ) {
                (XMAS::S, XMAS::A, XMAS::M, XMAS::X) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_diagonal_r_to_l(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col + 3],
                table[row + 1][col + 2],
                table[row + 2][col + 1],
                table[row + 3][col],
            ) {
                (XMAS::X, XMAS::M, XMAS::A, XMAS::S) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn count_diagonal_r_to_l_rev(table: &Vec<Vec<XMAS>>) -> usize {
    let mut count: usize = 0;
    for row in 0..table.len() - 3 {
        for col in 0..table[0].len() - 3 {
            match (
                table[row][col + 3],
                table[row + 1][col + 2],
                table[row + 2][col + 1],
                table[row + 3][col],
            ) {
                (XMAS::S, XMAS::A, XMAS::M, XMAS::X) => count += 1,
                _ => (),
            }
        }
    }
    count
}

fn parse_text(text: &str) -> Vec<Vec<XMAS>> {
    let mut table: Vec<Vec<XMAS>> = Vec::new();

    for line in text.lines() {
        let mut tmp: Vec<XMAS> = Vec::new();
        for ch in line.chars() {
            match ch {
                'X' => tmp.push(XMAS::X),
                'M' => tmp.push(XMAS::M),
                'A' => tmp.push(XMAS::A),
                'S' => tmp.push(XMAS::S),
                _ => panic!("unknown char {ch}"),
            };
        }
        table.push(tmp);
    }
    table
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_xmas_test() {
        let result = count_xmas(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 18);
    }

    #[test]
    fn count_horizontal_test() {
        let result = count_horizontal(&vec![
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::S, XMAS::A, XMAS::M, XMAS::X],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_horizontal_rev_test() {
        let result = count_horizontal_rev(&vec![
            vec![XMAS::S, XMAS::A, XMAS::M, XMAS::X],
            vec![XMAS::S, XMAS::A, XMAS::M, XMAS::X],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_vertical_test() {
        let result = count_vertical(&vec![
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::X],
            vec![XMAS::M, XMAS::X, XMAS::A, XMAS::X],
            vec![XMAS::A, XMAS::M, XMAS::A, XMAS::M],
            vec![XMAS::S, XMAS::A, XMAS::A, XMAS::A],
            vec![XMAS::X, XMAS::S, XMAS::A, XMAS::S],
        ]);
        assert_eq!(result, 3);
    }

    #[test]
    fn count_vertical_rev_test() {
        let result = count_vertical_rev(&vec![
            vec![XMAS::X, XMAS::S, XMAS::A, XMAS::X],
            vec![XMAS::S, XMAS::A, XMAS::A, XMAS::S],
            vec![XMAS::A, XMAS::M, XMAS::A, XMAS::A],
            vec![XMAS::M, XMAS::X, XMAS::A, XMAS::M],
            vec![XMAS::X, XMAS::S, XMAS::A, XMAS::X],
        ]);
        assert_eq!(result, 3);
    }

    #[test]
    fn count_diagonal_l_to_r_test() {
        let result = count_diagonal_l_to_r(&vec![
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_diagonal_l_to_r_rev_test() {
        let result = count_diagonal_l_to_r_rev(&vec![
            vec![XMAS::S, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::S, XMAS::A, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::A, XMAS::M, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::M, XMAS::X],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::X],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_diagonal_r_to_l_test() {
        let result = count_diagonal_r_to_l(&vec![
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::X],
            vec![XMAS::X, XMAS::M, XMAS::M, XMAS::X],
            vec![XMAS::X, XMAS::A, XMAS::M, XMAS::S],
            vec![XMAS::S, XMAS::A, XMAS::A, XMAS::S],
            vec![XMAS::S, XMAS::M, XMAS::A, XMAS::S],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_diagonal_r_to_l_rev_test() {
        let result = count_diagonal_r_to_l_rev(&vec![
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
            vec![XMAS::X, XMAS::M, XMAS::A, XMAS::S],
        ]);
        assert_eq!(result, 2);
    }

    #[test]
    fn count_mas_top_test() {
        let result = count_mas_top(&vec![
            vec![XMAS::M, XMAS::M, XMAS::M],
            vec![XMAS::X, XMAS::A, XMAS::A],
            vec![XMAS::S, XMAS::M, XMAS::S],
        ]);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_mas_left_test() {
        let result = count_mas_left(&vec![
            vec![XMAS::M, XMAS::M, XMAS::S],
            vec![XMAS::X, XMAS::A, XMAS::A],
            vec![XMAS::M, XMAS::M, XMAS::S],
        ]);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_mas_right_test() {
        let result = count_mas_right(&vec![
            vec![XMAS::S, XMAS::M, XMAS::M],
            vec![XMAS::X, XMAS::A, XMAS::A],
            vec![XMAS::S, XMAS::M, XMAS::M],
        ]);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_mas_bottom_test() {
        let result = count_mas_bottom(&vec![
            vec![XMAS::S, XMAS::M, XMAS::S],
            vec![XMAS::X, XMAS::A, XMAS::A],
            vec![XMAS::M, XMAS::M, XMAS::M],
        ]);
        assert_eq!(result, 1);
    }

    #[test]
    fn count_mas_test() {
        let result = count_mas(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, 9);
    }
}
