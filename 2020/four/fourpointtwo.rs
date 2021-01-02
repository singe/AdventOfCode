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
    if record.get(&field.to_string()) == None {
      return Err("Missing required fields");
    }
  }

  let error_val = "0".to_string();

  let byr = record.get("byr").unwrap_or(&error_val).parse().unwrap_or(0);
  if ! (1920..=2002).contains(&byr) {
    return Err("Birth year out of allowed range"); 
  }
  let iyr = record.get("iyr").unwrap_or(&error_val).parse().unwrap_or(0);
  if ! (2010..=2020).contains(&iyr) {
    return Err("Issue year out of allowed range"); 
  }
  let eyr = record.get("eyr").unwrap_or(&error_val).parse().unwrap_or(0);
  if ! (2020..=2030).contains(&eyr) {
    return Err("Expiration year out of allowed range"); 
  }

  let hgt = record.get("hgt").unwrap_or(&error_val);
  let hgttyp = &hgt[hgt.len()-2 .. ];
  let hgtnum = hgt[..hgt.len()-2].parse().unwrap_or(0);
  if hgttyp != "cm" && hgttyp != "in" {
    return Err("Height not in correct format");
  }
  match hgttyp {
    "cm" => if ! (150..=193).contains(&hgtnum) {
        return Err("Height out of allowed cm range");
      },
    "in" => if ! (59..=76).contains(&hgtnum) {
        return Err("Height out of allowed in range");
      },
    _ => (),
  }

  // should use regex here "^#[0-9a-f]{6}$" but didn't want to use an external crate yet
  let mut count = 0;
  let mut valid = true;
  for chr in record.get("hcl").unwrap_or(&error_val).chars() {
    match count {
      0 => if chr != '#' { valid = false; break; },
      1..=6 => if ! (('a'..='f').contains(&chr) || ('0'..='9').contains(&chr))
        { 
          valid = false; break; 
        },
      _ => { valid = false; break; },
    }
    count += 1;
  }
  if valid == false {
    return Err("Hair colour not valid");
  }

  let cols = ["amb","blu","brn","gry","grn","hzl","oth"];
  let eclclr: &str = &record.get("ecl").unwrap_or(&error_val);
  if ! cols.contains(&eclclr) {
    return Err("Eye colour not valid");
  }

  let pidstr = record.get("pid").unwrap_or(&error_val);
  if pidstr.len() != 9 {
    return Err("Passport number inappropriate length");
  }
  let pid;
  match pidstr.parse() {
    Ok(p) => pid = p,
    Err(_) => return Err("Passport number not a number"),
  }
  if ! (0..=999999999).contains(&pid) {
    return Err("Passport number not valid"); 
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
