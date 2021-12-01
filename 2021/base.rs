use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<String> {
  let mut db: Vec<String> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        println!("Hello World {:?}", result);
        db.push(result);
      }
    }
  }
  return db;
/*
  let lines;
  match read_lines(filename) {
    Ok(v) => lines = v,
    Err(e) => {
      println!("Error reading file: {}",e);
      return;
    },
  }
  for line in lines {
    let result;
    match line {
      Ok(v) => result = v,
      Err(e) => {
        println!("Error reading line: {}",e);
        return;
      },
    }
    println!("Hello World {}", result);
    db.push(result);
  }
*/
}

fn check(db: &Vec<String>) 
{
  for record in db {
    println!("Hello again World {:?}", record);
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
