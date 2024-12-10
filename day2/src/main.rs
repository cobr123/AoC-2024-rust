use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1().map(|safe_count| println!("{}", safe_count))?;
    part2().map(|safe_count| println!("{}", safe_count))
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

fn part2() -> io::Result<i32> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut safe_count = 0;

    for line in reader.lines() {
        let levels = line?
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        if is_safe_with_dampener(levels, 1) {
            safe_count += 1;
        }
    }
    Ok(safe_count)
}

fn is_safe(levels: Vec<i32>) -> bool {
    is_safe_with_dampener(levels, 0)
}

fn is_safe_with_dampener(levels: Vec<i32>, max_fails: i32) -> bool {
    if is_safe_with_dampener_impl(levels.clone(), max_fails) {
        true
    } else if max_fails > 0 {
        let mut levels_l = levels.clone();
        levels_l.remove(0);
        if is_safe_with_dampener_impl(levels_l, max_fails - 1) {
            true
        } else if max_fails > 0 {
            let mut levels_r = levels.clone();
            levels_r.remove(levels_r.len() - 1);
            if is_safe_with_dampener_impl(levels_r, max_fails - 1) {
                true
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn is_safe_with_dampener_impl(levels: Vec<i32>, max_fails: i32) -> bool {
    let increasing = levels[0] < levels[1];
    let mut prev = levels[0];
    for i in 1..levels.len() {
        let curr = levels[i];
        let diff = if increasing { curr - prev } else { prev - curr };
        if diff < 1 || diff > 3 {
            return if max_fails > 0 {
                let mut levels_l = levels.clone();
                levels_l.remove(i - 1);
                if is_safe_with_dampener_impl(levels_l, max_fails - 1) {
                    true
                } else {
                    let mut levels_r = levels.clone();
                    levels_r.remove(i);
                    is_safe_with_dampener_impl(levels_r, max_fails - 1)
                }
            } else {
                false
            };
        }
        prev = curr;
    }
    true
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

    #[test]
    fn is_safe_with_dampener_increasing_true() {
        let result = is_safe_with_dampener(vec![1, 3, 2, 4, 5], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn is_safe_with_dampener_increasing_true2() {
        let result = is_safe_with_dampener(vec![48, 46, 47, 49, 51, 54, 56], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn is_safe_with_dampener_increasing_false() {
        let result = is_safe_with_dampener(vec![1, 2, 7, 8, 9], 1);
        assert_eq!(result, false);
    }

    #[test]
    fn is_safe_with_dampener_decreasing_true() {
        let result = is_safe_with_dampener(vec![8, 6, 4, 4, 1], 1);
        assert_eq!(result, true);
    }

    #[test]
    fn is_safe_with_dampener_decreasing_false() {
        let result = is_safe_with_dampener(vec![9, 7, 6, 2, 1], 1);
        assert_eq!(result, false);
    }
}
