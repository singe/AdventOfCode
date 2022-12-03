use std::io::BufRead;

fn main() {
    let score: usize = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut res: char = ' ';
            for c in line[0..line.len() / 2].chars() {
                if let Some(_) = &line[line.len() / 2..].find(c) {
                    res = c;
                    break;
                }
            }
            assert!(res != ' ');
            match res {
                'a'..='z' => (res as usize - 'a' as usize) + 1,
                'A'..='Z' => (res as usize - 'A' as usize) + 27,
                _ => unreachable!(),
            }
        })
        .sum();
    println!("{score}");
}
