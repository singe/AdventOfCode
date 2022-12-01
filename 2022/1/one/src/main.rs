use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn parse(filename: &String) -> Vec<String> {
    let mut db = Vec::new();

    let file = File::open(&filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let result = line.unwrap();
        db.push(result);
    }

    db
}

fn check(db: Vec<String>)
{
    let mut biggest = 0;
    let mut tally = 0;
    for record in db {
        if record.len() == 0 {
            if tally > biggest {
                biggest = tally;
            }
            tally = 0;
        } else {
            tally += record.parse::<usize>().unwrap();
        }
    }
    println!("Total largest calories {biggest}");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db = parse(&args[1]);
    check(db);
}
