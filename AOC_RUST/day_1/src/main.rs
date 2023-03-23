use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").expect("Failed to open file");
    let lines = BufReader::new(file).lines().map(|l| l.unwrap().trim().to_string()).collect::<Vec<String>>();

    let mut totals = std::collections::HashMap::new();
    let mut current_elf = 1;
    for line in lines {
        if line == "" {
            current_elf += 1;
        } else {
            let calories = line.parse::<i32>().unwrap();
            let total = totals.entry(current_elf).or_insert(0);
            *total += calories;
        }
    }

    let mut sorted_totals: Vec<_> = totals.iter().collect();
    sorted_totals.sort_by_key(|x| x.1);
    sorted_totals.reverse();

    let top_three_totals: i32 = sorted_totals.iter().take(3).map(|x| x.1).sum();

    println!("Answer of Part One: {}", sorted_totals[0].1);
    println!("Answer of Part Two: {}", top_three_totals);
}

