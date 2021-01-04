use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<Vec<bool>> {
  let mut db: Vec<Vec<bool>> = Vec::new();
  db.push(vec![false;26]);
  let mut entry = db.len()-1;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        if result != "" {
          for chr in result.chars() {
            if chr.is_ascii_lowercase() {
              //a base 36 number system would be 0-9a-z
              //so a would be 10 and z would be 35
              //subtract 10 to map a to 0 and z to 25
              let value = chr.to_digit(36).unwrap() - 10;
              db[entry][value as usize] = true;
            }
          }
        } else { // new group
          db.push(vec![false;26]);
          entry = db.len()-1;
        }
      }
    }
  }
  return db;
}

fn check(record: &Vec<bool>) -> usize
{
  return record.iter().filter(|&n| *n == true).count();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let mut total = 0;
  for record in db {
    let value = check(&record);
    total += value;
  }
  println!("Total is {}",total);
}
