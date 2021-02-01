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

fn parse(filename: &String) -> Vec<usize> {
  let mut db: Vec<usize> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        db.push(result.parse().unwrap());
      }
    }
  }
  return db;
}

// A friend implemented it like this, I wanted to see if it was fast enough
// https://github.com/oxo42/adventcode2020/blob/master/day10.py
// Includes memoization
fn recurse(db: &Vec<usize>, index: usize, memo: &mut HashMap<usize,usize>) -> usize
{
  if memo.contains_key(&index) {
    return *memo.get(&index).unwrap();
  }

  if let Some(a) = db.last() {
    if index == *a {
      memo.insert(index,1);
      return 1;
    }
  }

  let mut x = 0;
  let mut i = 1;
  while i < 4 {
    if db.contains(&(index+i)) {
      x += recurse(&db, index+i, memo);
    }
    i += 1;
  }
  memo.insert(index,x);
  return x;
}

fn check(db: &mut Vec<usize>)
{
  db.sort();
  db.insert(0,0);
  db.push(db.last().unwrap()+3); // add final adapter

  let mut memo: HashMap<usize,usize> = HashMap::new();
  println!("{}",recurse(&db, 0, &mut memo));
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  check(&mut db);
}
