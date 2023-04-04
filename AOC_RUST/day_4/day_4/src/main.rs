use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let data: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut p = 0;
    for entry in &data {
        let parts: Vec<&str> = entry.split(",").collect();
        let first: Vec<i32> = parts[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let second: Vec<i32> = parts[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        if first[0] <= second[0] && first[1] >= second[1] || second[0] <= first[0] && second[1] >= first[1] {
            p += 1;
        }
    }

    println!("The answer of Part one is : {}", p);

    let mut p = 0;
    for entry in &data {
        let parts: Vec<&str> = entry.split(",").collect();
        let first: Vec<i32> = parts[0].split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        let second: Vec<i32> = parts[1].split("-").map(|x| x.parse::<i32>().unwrap()).collect();

        if (second[0]..=second[1]).contains(&first[0]) || (second[0]..=second[1]).contains(&first[1]) {
            p += 1;
        } else if (first[0]..=first[1]).contains(&second[0]) || (first[0]..=first[1]).contains(&second[1]) {
            p += 1;
        }
    }

    println!("The answer of Part two is : {}", p);
}

