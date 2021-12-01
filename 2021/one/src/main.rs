use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<usize> {
  let mut db: Vec<usize> = Vec::new();
  read_lines(filename)
    .unwrap()
    .for_each(|line| {
      db.push(line.unwrap().parse::<usize>().unwrap());
    });
  return db;
}

fn check(db: &Vec<usize>) 
{
  db.iter().enumerate()
    .for_each(|(i, record)| {
      match i {
        0 => println!("{} (N/A - no previous measurement)",record),
        _ if record > &db[i-1] => println!("{} (increased)",record),
        _ if record < &db[i-1] => println!("{} (decreased)",record),
        _ => (),
      }
    });
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
