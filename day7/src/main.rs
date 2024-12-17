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
    let sum = get_total_calibration(text.trim());

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
    let sum = get_total_calibration_with_concatenation(text.trim());

    Ok(sum)
}

fn get_total_calibration(text: &str) -> u128 {
    let equations = parse_text(text);
    sum_valid_equations(equations, &vec![Operator::Add, Operator::Multiply])
}

fn get_total_calibration_with_concatenation(text: &str) -> u128 {
    let equations = parse_text(text);
    sum_valid_equations(
        equations,
        &vec![Operator::Add, Operator::Multiply, Operator::Concatenation],
    )
}

fn sum_valid_equations(equations: Vec<Equation>, operators: &Vec<Operator>) -> u128 {
    equations
        .iter()
        .filter(|x| x.is_valid(operators))
        .map(|x| x.result)
        .sum()
}

#[derive(Debug)]
struct Equation {
    result: u128,
    values: Vec<u128>,
}

impl Equation {
    fn is_valid(&self, operators: &Vec<Operator>) -> bool {
        // println!("{} {:?}", self.result, self.values);
        nested(
            self.values.len() - 1,
            operators,
            self.result,
            &self.values,
            check_sum,
            &Vec::new(),
        )
    }
}

fn check_sum(result: u128, values: &Vec<u128>, operators: &Vec<&Operator>) -> bool {
    let mut sum = values[0];
    for i in 0..operators.len() {
        sum = match operators[i] {
            Operator::Add => sum + values[i + 1],
            Operator::Multiply => sum * values[i + 1],
            Operator::Concatenation => (sum.to_string() + values[i + 1].to_string().as_str())
                .parse::<u128>()
                .unwrap(),
        };
    }
    if result == sum {
        // println!("{} {} {:?} {:?}", result, sum, values, operators);
        true
    } else {
        false
    }
}

fn nested(
    left: usize,
    operators: &Vec<Operator>,
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, &Vec<&Operator>) -> bool,
    acc: &Vec<&Operator>,
) -> bool {
    if left == 0 {
        return f(result, values, acc);
    } else {
        for operator in operators {
            let mut new_acc: Vec<&Operator> = vec![&operator];
            new_acc.extend(acc);
            if nested(left - 1, operators, result, values, f, &new_acc) {
                return true;
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

fn parse_text(text: &str) -> Vec<Equation> {
    let mut equations = Vec::new();
    for line in text.lines() {
        let parts: Vec<_> = line.split(": ").collect();
        assert_eq!(2, parts.len());
        let result = parts[0].parse::<u128>().unwrap();
        let values = parts[1]
            .split_ascii_whitespace()
            .map(|x| x.parse::<u128>().unwrap())
            .collect();
        equations.push(Equation { result, values });
    }
    equations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_total_calibration_test() {
        let result = get_total_calibration(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 3749);
    }

    #[test]
    fn get_total_calibration_bigint_test() {
        let result = get_total_calibration(
            "49162643986: 42 153 8 9 5 3 849 8 8
8804602981773: 38 662 7 5 298 1 7 73
4113176424: 892 50 50 2 8 7 329
1117038228: 2 797 7 4 8 3 8 140 2 86
13544236: 281 482 17 19
69460233: 3 1 193 6 465 2 8 8 3 3 1",
        );
        assert_eq!(result, 69460233);
    }

    #[test]
    fn get_total_calibration_with_concatenation_test() {
        let result = get_total_calibration_with_concatenation(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, 11387);
    }
}
