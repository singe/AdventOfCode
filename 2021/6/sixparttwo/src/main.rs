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
  let mut db: Vec<usize> = vec![0;9];
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        result.split(",")
          .for_each(|x| db[x.parse::<usize>().unwrap() as usize] += 1);
      }
    }
  }
  return db;
}

fn check(db: &mut Vec<usize>) 
{
  println!("After 00 days {:02}, {:?}",db.iter().sum::<usize>(),db);
  let days = 256;

  for day in 1..=days {
    let tmp = db[0];
    db.remove(0);
    db.extend_from_slice(&[tmp]);
    db[6] += tmp;
    //println!("After {:02} days {:02}, {:?}",day,db.iter().sum::<usize>(),db);
  }

  println!("{} fish",db.iter().sum::<usize>());
}


fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  check(&mut db);
}
