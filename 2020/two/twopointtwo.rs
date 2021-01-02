use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct Policy {
  min: i32,
  max: i32,
  chr: String,
  pwd: String,
}

fn parse(filename: &String) -> Vec<Policy>{
  let mut db: Vec<Policy> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        let (mut count, mut min, mut max, mut chr, mut pwd) = (0, "", "", "", "");
        for s in result.split(&['-', ' ', ':'][..]) {
          match count {
            0 => min = s,
            1 => max = s,
            2 => chr = s,
            4 => pwd = s,
            _ => (),
          }
          count += 1;
        }
        let pol: Policy = Policy {
          min: min.parse().unwrap_or(-1),
          max: max.parse().unwrap_or(-1),
          chr: chr.to_string(),
          pwd: pwd.trim().to_string()
        };
        db.push(pol);
      }
    }
  }
  return db;
}

fn check(pol: &Policy) -> Result<usize,&str>
{
  if pol.max == -1 || pol.min == -1 {
    return Err("Error parsing policy.");
  }
  let first = pol.pwd.get(pol.min as usize-1..pol.min as usize).unwrap_or("xx");
  let second = pol.pwd.get(pol.max as usize-1..pol.max as usize).unwrap_or("xx");
  if first == "xx" || second == "xx" {
    return Err("Error parsing password.");
  }
  if first != pol.chr && second != pol.chr {
    return Err("Neither character matches.");
  } else if first == pol.chr && second == pol.chr {
    return Err("Both positions match.");
  } else if first == pol.chr || second == pol.chr {
    return Ok(1)
  }
  Err("Something went wrong")
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  for record in db {
    match check(&record) {
      Err(e) => println!("[*] Password {} violated policy {}-{} {}: {}", record.pwd, record.min, record.max, record.chr, e),
      Ok(_) => println!("[+] Password {} meets policy {}-{} {}", record.pwd, record.min, record.max, record.chr), 
    }
  }
}
