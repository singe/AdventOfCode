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
        result.split(",").for_each(|x| db.push(x.parse::<usize>().unwrap()));
      }
    }
  }
  db.sort();
  return db;
}

fn check(db: &Vec<usize>) 
{
  let mut shortest = (2147483647,2147483647);
  let mut cost = 0;
  for i in 0..=db[db.len()-1] {
    cost = 0;
    for j in 0..db.len() {
      let moves =  (i as i32 - db[j] as i32).abs();
      cost += (moves * (moves + 1))/2;
      //println!("Move from {} to {}: {} fuel",db[j],i,(i as i32 - db[j] as i32).abs());
    }
    println!("Total cost for pos {} is {}",i,cost);
    if cost < shortest.1 {
      shortest.1 = cost;
      shortest.0 = i;
    }
  }
  println!("Shortest pos is {} at cost {}",shortest.0,shortest.1);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
