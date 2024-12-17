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
    let sum = get_total_calibration(text.trim());

    Ok(sum)
}

fn get_total_calibration(text: &str) -> u128 {
    let equations = parse_text(text);
    sum_valid_equations(equations)
}

fn sum_valid_equations(equations: Vec<Equation>) -> u128 {
    equations
        .iter()
        .filter(|x| x.is_valid())
        .map(|x| x.result)
        .sum()
}

#[derive(Debug)]
struct Equation {
    result: u128,
    values: Vec<u128>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        // println!("{} {:?}", self.result, self.values);
        match self.values.len() - 1 {
            1 => nested1(self.result, &self.values, check_sum),
            2 => nested2(self.result, &self.values, check_sum),
            3 => nested3(self.result, &self.values, check_sum),
            4 => nested4(self.result, &self.values, check_sum),
            5 => nested5(self.result, &self.values, check_sum),
            6 => nested6(self.result, &self.values, check_sum),
            7 => nested7(self.result, &self.values, check_sum),
            8 => nested8(self.result, &self.values, check_sum),
            9 => nested9(self.result, &self.values, check_sum),
            10 => nested10(self.result, &self.values, check_sum),
            11 => nested11(self.result, &self.values, check_sum),
            other => panic!("can't handle {other} operators"),
        }
    }
}

fn check_sum(result: u128, values: &Vec<u128>, operators: Vec<&Operator>) -> bool {
    let mut sum = values[0];
    for i in 0..operators.len() {
        sum = match operators[i] {
            Operator::Add => sum + values[i + 1],
            Operator::Multiply => sum * values[i + 1],
        };
    }
    if result == sum {
        // println!("{} {} {:?} {:?}", result, sum, values, operators);
        true
    } else {
        false
    }
}

fn nested1(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        if f(result, values, vec![o1]) {
            return true;
        }
    }
    false
}

fn nested2(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            if f(result, values, vec![o1, o2]) {
                return true;
            }
        }
    }
    false
}

fn nested3(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                if f(result, values, vec![o1, o2, o3]) {
                    return true;
                }
            }
        }
    }
    false
}

fn nested4(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    if f(result, values, vec![o1, o2, o3, o4]) {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn nested5(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        if f(result, values, vec![o1, o2, o3, o4, o5]) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested6(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            if f(result, values, vec![o1, o2, o3, o4, o5, o6]) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested7(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            for o7 in &Operator::variants() {
                                if f(result, values, vec![o1, o2, o3, o4, o5, o6, o7]) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested8(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            for o7 in &Operator::variants() {
                                for o8 in &Operator::variants() {
                                    if f(result, values, vec![o1, o2, o3, o4, o5, o6, o7, o8]) {
                                        return true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested9(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            for o7 in &Operator::variants() {
                                for o8 in &Operator::variants() {
                                    for o9 in &Operator::variants() {
                                        if f(
                                            result,
                                            values,
                                            vec![o1, o2, o3, o4, o5, o6, o7, o8, o9],
                                        ) {
                                            return true;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested10(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            for o7 in &Operator::variants() {
                                for o8 in &Operator::variants() {
                                    for o9 in &Operator::variants() {
                                        for o10 in &Operator::variants() {
                                            if f(
                                                result,
                                                values,
                                                vec![o1, o2, o3, o4, o5, o6, o7, o8, o9, o10],
                                            ) {
                                                return true;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn nested11(
    result: u128,
    values: &Vec<u128>,
    f: fn(u128, &Vec<u128>, Vec<&Operator>) -> bool,
) -> bool {
    for o1 in &Operator::variants() {
        for o2 in &Operator::variants() {
            for o3 in &Operator::variants() {
                for o4 in &Operator::variants() {
                    for o5 in &Operator::variants() {
                        for o6 in &Operator::variants() {
                            for o7 in &Operator::variants() {
                                for o8 in &Operator::variants() {
                                    for o9 in &Operator::variants() {
                                        for o10 in &Operator::variants() {
                                            for o11 in &Operator::variants() {
                                                if f(
                                                    result,
                                                    values,
                                                    vec![
                                                        o1, o2, o3, o4, o5, o6, o7, o8, o9, o10,
                                                        o11,
                                                    ],
                                                ) {
                                                    return true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}

impl Operator {
    fn variants() -> Vec<Operator> {
        vec![Operator::Add, Operator::Multiply]
    }
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
}
