use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = env::args().nth(1).expect("Missing file");
    let file = File::open(&filename).expect("Unable to open file");

    let mut biggest: Vec<usize> = Vec::new();
    io::BufReader::new(file)
        .lines()
        .map(|line| match line.expect("Line parse error").parse() {
            Ok(x) => x,
            Err(_) => 0,
        })
        .reduce(|acc, item| {
            if item == 0 {
                biggest.push(acc);
                0
            } else {
                acc + item
            }
        });
    biggest.sort_unstable_by(|a, b| b.cmp(a));
    let result: usize = biggest.iter().take(3).sum();
    println!("Total largest calories {result:?}");
}
