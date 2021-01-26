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
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        //println!("Hello World {:?}", result);
        db.push(result.parse().unwrap());
      }
    }
  }
  return db;
}

fn check(db: &Vec<usize>, preamlen: usize) -> Result<usize,&str>
{
  if preamlen < 2 || preamlen >= db.len() {
    return Err("Impossible preamble")
  }
  let mut valid = true;
  for window in db.windows(preamlen+1) {
    let mut found = false;
    //check last num is list against preamble
    let num = window.last().unwrap();
    let mut x = 0;

    while x < window.len()-2 && !found {
      if window[x] >= *num {
        x += 1;
        continue;
      }
      println!("num {} check {} window {:?}",num,num-window[x],window);
      found = window.contains(&(num-window[x]));
      x += 1;
    }
    if !found {
      println!("Bad {}",num);
      valid = false;
    } else {
      println!("Good {}",num);
    }
  }
  if !valid {
    return Err("Chain not valid");
  }
  return Ok(0);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let preamlen = &args[2].parse().unwrap();
  println!("{:?}",check(&db,*preamlen));
}
