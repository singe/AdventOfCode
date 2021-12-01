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
  read_lines(filename)
    .unwrap()
    .for_each(|line| {
      db.push(line.unwrap().parse::<usize>().unwrap());
    });
  return db;
}

fn check(db: &Vec<usize>) 
{
  let mut count = 0;
  let win: Vec<&[usize]> = db.windows(3).collect();
  win.windows(2)
    .for_each(|window| {
      let prev: usize = window[0].iter().sum();
      let next: usize = window[1].iter().sum();
      if prev < next { println!("{} (increased)",next); count += 1; }
      else { println!("{} (decreased)",next); }
    });
  println!("{} measurements are larger than the previous",count);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
