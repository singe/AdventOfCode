use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<Vec<String>> {
  let mut db: Vec<Vec<String>> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        db.push(result.split(" |").nth(1).unwrap().split(" ").map(|s| s.to_string()).collect::<Vec<String>>());
      }
    }
  }
  return db;
}

fn check(db: &Vec<Vec<String>>) 
{
  let mut count = 0;
  db.iter()
    .for_each(|line| 
              line.iter()
                  .for_each(|s| {
                    match s.len() {
                      2 => count += 1,
                      4 => count += 1,
                      3 => count += 1,
                      7 => count += 1,
                      _ => (),
                    }
                  }));
  println!("Total: {}",count);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
