use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::convert::TryInto;

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

fn parse(filename: &String) -> Vec<Policy> {
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
            3 => (), //empty due to colon then space
            4 => pwd = s,
            _ => println!("Something has gone wrong parsing: {}",result),
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

fn check(pol: &Policy) -> Result<i32,String>
{
  let occurances: i32 = pol.pwd.matches(&pol.chr).count().try_into().unwrap_or(-1);
  if pol.max == -1 || pol.min == -1 || occurances == -1 {
    return Err(format!("Error parsing policy or password."));
  }
  if occurances > pol.max {
    return Err(format!("Password exceeds maximum number of '{}' characters: {}", pol.chr, pol.max.to_string()));
  } else if occurances < pol.min {
    return Err(format!("Password doesn't meet minimum number of '{}' characters: {}", pol.chr, pol.min.to_string()));
  }
  return Ok(1)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db: Vec<Policy> = parse(&args[1]);
  for record in db {
    match check(&record) {
      Err(e) => println!("[*] Password {} violated policy {}-{} {}: {}", record.pwd, record.min, record.max, record.chr, e),
      Ok(_) => println!("[+] Password {} meets policy {}-{} {}", record.pwd, record.min, record.max, record.chr), 
    }
  }
}
