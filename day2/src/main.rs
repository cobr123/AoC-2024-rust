use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|safe_count| println!("{}", safe_count))
}

fn part1() -> io::Result<i32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;

    for line in reader.lines() {
        let levels = line?
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe(levels) {
            safe_count += 1;
        }
    }
    Ok(safe_count)
}

fn is_safe(levels: Vec<i32>) -> bool {
    let increasing = levels[0] < levels[1];
    let mut prev = levels[0];
    let mut safe = true;
    for i in 1..levels.len() {
        let curr = levels[i];
        let diff = if increasing { curr - prev } else { prev - curr };
        if diff < 1 || diff > 3 {
            safe = false;
            break;
        }
        prev = curr;
    }
    safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_safe_increasing_true() {
        let result = is_safe(vec![1, 3, 6, 7, 9]);
        assert_eq!(result, true);
    }

    #[test]
    fn is_safe_increasing_false() {
        let result = is_safe(vec![1, 2, 7, 8, 9]);
        assert_eq!(result, false);
    }

    #[test]
    fn is_safe_decreasing_true() {
        let result = is_safe(vec![7, 6, 4, 2, 1]);
        assert_eq!(result, true);
    }

    #[test]
    fn is_safe_decreasing_false() {
        let result = is_safe(vec![9, 7, 6, 2, 1]);
        assert_eq!(result, false);
    }
}
