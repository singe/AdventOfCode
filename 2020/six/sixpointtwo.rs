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
  let mut person = vec![false;26]; // tmp record per person/line
  let mut group = 0; // index to db
  let mut newgrp = true; // first person in group indicator
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        if result == "" { // blank line is a new group
          newgrp = true;
        } else { // new person
          for chr in result.chars() {
            if chr.is_ascii_lowercase() {
              //a base 36 number system would be 0-9a-z
              //so a would be 10 and z would be 35
              //subtract 10 to map a to 0 and z to 25
              let value = chr.to_digit(36).unwrap() - 10;
              person[value as usize] = true; 
            }
          }
          if newgrp { // first person in a group sets initial vec
            db.push(person.clone());
            group = db.len()-1;
            newgrp = false;
          } else {
            for count in 0..person.len() {
              db[group][count] = db[group][count] && person[count];
            }
          }
          person = vec![false;26]; // reset person
        }
      }
    }
  }
  return db;
}

fn check(record: &Vec<bool>) -> usize
{
  return record.iter().filter(|&n| *n).count();
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let mut total = 0;
  for record in db {
    //println!("{:?}",record);
    let value = check(&record);
    println!("Value is {}",value);
    total += value;
  }
  println!("Total is {}",total);
}
