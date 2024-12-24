use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|count| println!("{}", count))
}

fn part1() -> io::Result<u128> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let count = count(text.trim());

    Ok(count)
}

fn count(line: &str) -> u128 {
    parse_line(blink(line, 25).as_str()).len() as u128
}

fn parse_line(line: &str) -> Vec<u128> {
    line.split_ascii_whitespace()
        .map(|str| str.parse::<u128>().unwrap())
        .collect()
}

fn blink(line: &str, left: usize) -> String {
    let mut numbers = parse_line(line);
    for _ in 0..left {
        let mut idx = 0;
        while idx < numbers.len() {
            if numbers[idx] == 0 {
                numbers[idx] = 1;
            } else if numbers[idx].to_string().len() % 2 == 0 {
                let n_string = numbers[idx].to_string();
                let (left_half, right_half) = n_string.split_at(n_string.len() / 2);
                // println!("{} = {} | {}", n_string, left_half, right_half);
                numbers.insert(idx, left_half.parse::<u128>().unwrap());
                idx += 1;
                numbers[idx] = right_half.parse::<u128>().unwrap();
            } else {
                numbers[idx] = numbers[idx] * 2024;
            }
            idx += 1;
        }
        // println!("{:?}",numbers);
    }
    numbers
        .iter()
        .map(|x| format!("{x}"))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blink1_test() {
        let result = blink("125 17", 1);
        assert_eq!(result, "253000 1 7");
    }

    #[test]
    fn blink12_test() {
        let result = blink("253000 1 7", 1);
        assert_eq!(result, "253 0 2024 14168");
    }

    #[test]
    fn blink2_test() {
        let result = blink("125 17", 2);
        assert_eq!(result, "253 0 2024 14168");
    }

    #[test]
    fn blink3_test() {
        let result = blink("125 17", 3);
        assert_eq!(result, "512072 1 20 24 28676032");
    }

    #[test]
    fn blink4_test() {
        let result = blink("125 17", 4);
        assert_eq!(result, "512 72 2024 2 0 2 4 2867 6032");
    }

    #[test]
    fn blink5_test() {
        let result = blink("125 17", 5);
        assert_eq!(result, "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32");
    }

    #[test]
    fn blink6_test() {
        let result = blink("125 17", 6);
        assert_eq!(
            result,
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
        );
    }

    #[test]
    fn blink25_test() {
        let result = blink("125 17", 25);
        assert_eq!(parse_line(result.as_str()).len(), 55312);
    }
}
