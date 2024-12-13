use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|count| println!("{}", count))?;
    part2().map(|count| println!("{}", count))
}

fn part1() -> io::Result<u32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let sum = sum_correct_middle(text.trim());

    Ok(sum)
}

fn part2() -> io::Result<u32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let text = reader
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");
    let sum = sum_incorrect_middle_after_correction(text.trim());

    Ok(sum)
}

fn sum_correct_middle(text: &str) -> u32 {
    let (rules, updates) = parse_text(text);
    updates
        .iter()
        .filter(|x| is_correct(x, &rules))
        .map(|x| {
            let mid = (x.len() - 1) / 2;
            x[mid]
        })
        .sum()
}

fn sum_incorrect_middle_after_correction(text: &str) -> u32 {
    let (rules, updates) = parse_text(text);
    updates
        .iter()
        .filter(|x| !is_correct(x, &rules))
        .map(|x| {
            let mid = (x.len() - 1) / 2;
            let corrected = correct_by_rules(x, &rules);
            corrected[mid]
        })
        .sum()
}

fn correct_by_rules(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut corrected = update.clone();
    let mut found = true;
    while found {
        found = false;
        for (a, b) in rules {
            for i in 0..corrected.len() {
                if corrected[i] == *a {
                    for k in (0..i).rev() {
                        if corrected[k] == *b {
                            corrected.remove(i);
                            corrected.insert(k, *a);
                            found = true;
                            break;
                        }
                    }
                    break;
                }
            }
        }
    }
    corrected
}

fn is_correct(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for i in 0..update.len() - 1 {
        let page = update[i];
        let filtered_rules: HashSet<u32> = rules
            .iter()
            .filter_map(|(a, b)| if *b == page { Some(*a) } else { None })
            .collect();
        for k in i..update.len() {
            if filtered_rules.contains(&update[k]) {
                return false;
            }
        }
    }
    true
}

fn parse_text(text: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules: Vec<(u32, u32)> = Vec::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut read_rules = true;

    for line in text.lines() {
        if line.is_empty() {
            read_rules = false;
        } else if read_rules {
            let parts = line.split("|").collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);
            let (a, b) = (
                parts[0].parse::<u32>().unwrap(),
                parts[1].parse::<u32>().unwrap(),
            );
            rules.push((a, b));
        } else {
            let parts = line
                .split(",")
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            updates.push(parts);
        }
    }
    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_correct_middle_test() {
        let result = sum_correct_middle(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 143);
    }

    #[test]
    fn sum_incorrect_middle_after_correction_test() {
        let result = sum_incorrect_middle_after_correction(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, 123);
    }
}
