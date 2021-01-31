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

// this is too slow to solve the input, but it works on the examples
// it gave me the lookup table used in the final tenpointtwov2
fn recurse(db: &Vec<usize>, index: usize, chain: &Vec<usize>)
{
  let mut newchain = chain.clone();
  newchain.push(db[index]);
  if index == db.len()-1 {
    println!("{:?}",newchain);
    return;
  } else {
    let mut i = 1;
    while i < 4 && index+i < db.len() {
      if db[index+i]-db[index] < 4 {
        recurse(&db, index+i, &newchain);
      }
      i += 1;
    }
  }
}

fn check(db: &mut Vec<usize>)
{
  db.sort();
  db.insert(0,0);
  db.push(db.last().unwrap()+3); // add final adapter

  let chain: Vec<usize> = vec![]; 
  recurse(&db, 0, &chain);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  check(&mut db);
}
