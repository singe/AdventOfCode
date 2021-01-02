use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<String> {
  let mut db: Vec<String> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        db.push(result);
      }
    }
  }
  return db;
}

fn check(record: &String) -> (usize,usize,usize) 
{
  // rows
  let (mut base, mut len) = (0, 128);
  for chr in record[0..7].chars() {
    len = len/2;
    match chr {
      //'F' => (),
      'B' => base += len,
      _ => (),
    }
  }
  let row = base;
  //cols
  let (mut base, mut len) = (0, 8);
  for chr in record[7..10].chars() {
    len = len/2;
    match chr {
      //'L' => (),
      'R' => base += len,
      _ => (),
    }
  }
  let col = base;
  let seatid = row*8 + col;
  println!("record is {} row is {} col is {} seatid is {}",record,row,col,seatid);
  return (row, col, seatid);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let mut maxseatid = 0;
  for record in db {
    let (_row, _col, seatid) = check(&record);
    if seatid > maxseatid {
      maxseatid = seatid;
    }
  }
  println!("Largest seatid is {}",maxseatid);
}
