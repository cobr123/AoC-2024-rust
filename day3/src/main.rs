use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|sum| println!("{}", sum))?;
    part2().map(|sum| println!("{}", sum))
}

fn part1() -> io::Result<i32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut sum = 0;
    for line in reader.lines() {
        sum += parse_line(line?.as_str());
    }

    Ok(sum)
}

fn part2() -> io::Result<i32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("");

    let sum = parse_line_with_do_and_dont(text.as_str());

    Ok(sum)
}

fn parse_line(line: &str) -> i32 {
    line.split("mul(")
        .map(|str| str.split(")").next().unwrap())
        .map(|str| str.split(","))
        .filter_map(|v| parse_mul(v.collect()))
        .sum()
}

fn parse_line_with_do_and_dont(line: &str) -> i32 {
    let parts: Vec<&str> = line.split("don't()").collect();
    let mut text = parts[0].to_owned();
    for i in 1..parts.len() {
        let mut tmp = parts[i].split("do()").collect::<Vec<&str>>();
        tmp.remove(0);
        text += tmp.join("").as_str();
    }
    parse_line(text.as_str())
}

fn parse_mul(parts: Vec<&str>) -> Option<i32> {
    if parts.len() == 2 {
        match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            (Ok(a), Ok(b)) => Option::Some(a * b),
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_line_test() {
        let result =
            parse_line("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        assert_eq!(result, 161);
    }

    #[test]
    fn parse_line_with_do_and_dont_test() {
        let result = parse_line_with_do_and_dont(
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        );
        assert_eq!(result, 48);
    }
}
