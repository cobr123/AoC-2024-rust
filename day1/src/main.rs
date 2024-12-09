use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    part1()?;
    part2()
}

fn part1() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut arr_l: Vec<i32> = Vec::new();
    let mut arr_r: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let l = line?;
        let parts = l.split("   ").collect::<Vec<_>>();
        let (a, b) = (
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        );
        arr_l.push(a);
        arr_r.push(b);
    }
    arr_l.sort();
    arr_r.sort();

    let mut sum = 0;
    for i in 0..arr_l.len() {
        let a = arr_l[i];
        let b = arr_r[i];
        let diff = if a > b { a - b } else { b - a };
        sum += diff;
    }
    println!("{}", sum);

    Ok(())
}

fn part2() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut arr_l: Vec<i32> = Vec::new();
    let mut arr_r: BTreeMap<i32, i32> = BTreeMap::new();

    for line in reader.lines() {
        let l = line?;
        let parts = l.split("   ").collect::<Vec<_>>();
        let (a, b) = (
            parts[0].parse::<i32>().unwrap(),
            parts[1].parse::<i32>().unwrap(),
        );
        arr_l.push(a);

        match arr_r.get(&b) {
            Some(count) => arr_r.insert(b, count + 1),
            None => arr_r.insert(b, 1),
        };
    }

    let mut sum = 0;
    for a in arr_l {
        let count = arr_r.get(&a).unwrap_or(&0);
        sum += a * count;
    }
    println!("{}", sum);

    Ok(())
}
