use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let filename = env::args().nth(1).expect("Missing file");
    let file = File::open(&filename).expect("Unable to open file");

    let mut biggest = 0;
    io::BufReader::new(file)
        .lines()
        .map(
            |line| match line.expect("Line parse error").parse() {
                Ok(x) => x,
                Err(_) => 0,
            },
        )
        .reduce(|acc, item| {
            if acc > biggest {
                biggest = acc;
            }
            if item == 0 {
                0
            } else {
                acc + item
            }
        });
    println!("Total largest calories {biggest}");
}
