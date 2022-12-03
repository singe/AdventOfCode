use std::io::BufRead;

fn main() {
    let score: usize = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut res = 0;
            let one = &line[0..line.len() / 2];
            let two = &line[line.len() / 2..];
            if let Some(i) = two.find(|c| one.contains(c)) {
                res = two.chars().nth(i).unwrap() as usize;
            }
            assert!(res != 0);
            match res {
                97..=122 => (res - 97) + 1,
                65..=90 => (res - 65) + 27,
                _ => unreachable!(),
            }
        })
        .sum();
    println!("{score}");
}
