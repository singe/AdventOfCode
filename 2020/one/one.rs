use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse<P>(filename: P) -> Vec<i32>
where P: AsRef<Path>, {
  let mut db: Vec<i32> = Vec::new();

  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(expense) = line {
        if let Ok(amount) = expense.parse() {
          db.push(amount);
        }
      }
    }
  }
  return db;
}

fn check(db: &Vec<i32>) 
{
  let mut result: i32 = 0;
  for amount in db {
    println!("Checking: {}", amount.to_string());
    for other in db {
      if (amount + other) == 2020 {
        result = *other;
        println!("Found: {}. Result: {}", other.to_string(), (amount * other).to_string());
        break;
      }
    }
    if result != 0 { break; }
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let expenses = parse(&args[1]);
  check(&expenses);

}
