use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<HashMap<String,String>> {
  let mut db = Vec::new();
  db.push(HashMap::new());
  let mut entry = db.len()-1;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        for record in result.split(' ') {
          for pair in record.split(':').collect::<Vec<&str>>().chunks(2) {
            if pair.len() == 2 {
              db[entry].insert(pair[0].to_string(),pair[1].to_string());
            } else {
              db.push(HashMap::new());
              entry = db.len()-1;
            }
          }
        }
      }
    }
  }
  return db;
}

fn check(record: &HashMap<String,String>) -> Result<&str,&str>
{
  let fields = ["byr","iyr","eyr","hgt","hcl","ecl","pid"];
  if record.len() < 7 {
    return Err("Not enough total fields");
  }
  for field in &fields {
    //if let None(e) = record.get(&field.to_string()) {
    if record.get(&field.to_string()) == None {
      return Err("Missing required fields");
    }
  }
  if record.get("cid") == None {
    return Ok("Valid - North Pole");
  }
  return Ok("Valid");
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  for record in db {
    println!("{:?}",check(&record));
  }
}
