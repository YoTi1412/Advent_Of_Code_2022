use std::collections::HashSet;
use std::fs;

fn main() {
    let ascii_letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let input = fs::read_to_string("input.txt").unwrap();
    let rucksack: Vec<&str> = input.trim().split('\n').collect();

    let mut total_sum = 0;
    for r in &rucksack {
        let half = r.len() / 2;
        let left: HashSet<char> = r[..half].chars().collect();
        let right: HashSet<char> = r[half..].chars().collect();

        for (priority, char) in ascii_letters.chars().enumerate() {
            if left.contains(&char) && right.contains(&char) {
                total_sum += priority + 1;
            }
        }
    }
    println!("Answer of part one is: {}", total_sum);

    let mut total_sum = 0;
    let mut j = 3;
    for i in (0..rucksack.len()).step_by(3) {
        let r = &rucksack[i..j];
        j += 3;

        for (priority, char) in ascii_letters.chars().enumerate() {
            if r.iter().all(|s| s.contains(char)) {
                total_sum += priority + 1;
            }
        }
    }
    println!("Answer of part two is: {}", total_sum);
}
