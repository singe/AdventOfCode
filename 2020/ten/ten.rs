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
        db.push(result.parse().unwrap());
      }
    }
  }
  return db;
}

fn check(db: &mut Vec<usize>) -> usize
{
  db.sort();
  db.insert(0,0);
  //println!("{:?}",db);
  db.push(db.last().unwrap()+3); // add final adapter
  let mut threes = 0;
  let mut twos = 0;
  let mut ones = 0;
  let mut i = 1;
  while i < db.len() {
    match db[i] {
      e if e == db[i-1] +1 => ones += 1,
      e if e == db[i-1] +2 => twos += 1,
      e if e == db[i-1] +3 => threes += 1,
      _ => (), 
    }
    println!("{}",db[i]-db[i-1]);
    i += 1;
  }
  println!("ones {} twos {} threes {}",ones,twos,threes);
  return threes*ones;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  println!("{}",check(&mut db));
}
