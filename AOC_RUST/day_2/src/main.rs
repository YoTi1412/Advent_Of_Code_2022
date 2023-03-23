use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let rounds: Vec<&str> = input.trim().split('\n').collect();

    let out = vec![
        ("A X", 4), ("A Y", 8), ("A Z", 3),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 7), ("C Y", 2), ("C Z", 6),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    let mut total_pts = 0;
    for r in &rounds {
        total_pts += out[r];
    }

    let out2 = vec![
        ("A X", 3), ("A Y", 4), ("A Z", 8),
        ("B X", 1), ("B Y", 5), ("B Z", 9),
        ("C X", 2), ("C Y", 6), ("C Z", 7),
    ].iter().cloned().collect::<std::collections::HashMap<_, _>>();

    let mut total_pts2 = 0;
    for r in &rounds {
        total_pts2 += out2[r];
    }

    println!("Answer of Part one : {}", total_pts);
    println!("Answer of Part two : {}", total_pts2);
}


