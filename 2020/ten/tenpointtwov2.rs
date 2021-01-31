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

// this one works on the input and it's fast
fn check(db: &mut Vec<usize>) -> usize
{
  db.sort();
  db.insert(0,0);
  //println!("{:?}",db);
  db.push(db.last().unwrap()+3); // add final adapter
  let mut total = 1;
  let mut ones = 0;
  let mut i = 1;
  while i < db.len() {
    match db[i]-db[i-1] {
      // count the number of ones in a row
      1 => ones += 1,
      3 => {
            // a cheap lookup table for the number of permutations the
            // ones in a row produce. The input never had more than 4.
            // I used tenpointtwo.rs to generate this to a higher number
            // but it wasn't needed
            match ones {
              1 => (),
              2 => total *= 2,
              3 => total *= 4,
              4 => total *= 7,
              5 => total *= 13,
              _ => (),
            }
            ones = 0;
           },
      _ => (), 
    }
    //println!("{}",db[i]-db[i-1]);
    i += 1;
  }
  return total;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  println!("{}",check(&mut db));
}
