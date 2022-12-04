use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut score = 0;
    while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
        let set1: HashSet<char> = line1.chars().collect();
        let set2: HashSet<char> = line2.chars().collect();
        let set3: HashSet<char> = line3.chars().collect();
        let common: Vec<char> = set1
            .intersection(&set2)
            .cloned()
            .collect::<HashSet<char>>()
            .intersection(&set3)
            .cloned()
            .collect();
        let badge = common[0] as usize;
        score += match badge {
            97..=122 => (badge - 97) + 1,
            65..=90 => (badge - 65) + 27,
            _ => unreachable!(),
        };
    }
    println!("{score}");
}
