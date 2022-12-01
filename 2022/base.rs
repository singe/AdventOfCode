use std::env;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, BufRead};

fn parse(filename: &String) -> Vec<String> {
    let mut db = Vec::new();

    let file = File::open(&filename).unwrap();
    let lines = io::BufReader::new(file).lines();

    for line in lines {
        let result = line.unwrap();
        println!("Hello World {:?}", result);
        db.push(result);
    }

    db
}

fn check<Iter>(db: Iter)
where
    Iter: IntoIterator,
    <Iter as IntoIterator>::Item: Debug,
{
    for record in db {
        println!("Hello again World {:?}", record);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let db = parse(&args[1]);
    check(db);
}
