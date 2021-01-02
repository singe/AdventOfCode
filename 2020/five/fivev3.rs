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

fn check(record: &String) -> (u16,u16,u16) 
{
  // You can convert the whole thing into a single binary number
  // i.e. FFBFBBBRLR is 10111101 with the last 3 LSB being col
  let mut seatid: u16 = 0;
  let mut count = 0;
  // ASCII 'F' & 'B' can be turned to binary based on the 3rd LSB
  // Likewise 'R' & 'L' can be done in the same way
  for x in record.bytes().rev() {
    let mut y: u16 = x as u16 & 4; // 4 is 100 in binary, we only was that bit
    y >>= 2; // shift it to the 1st LSD
    y ^= 1; // flip 1 to 0 and 0 to 1
    y <<= count; // put it in the right position
    seatid = seatid | y; // add it to the final number
    count += 1;
  }
  let row = seatid >> 3; // row is the first 7 bits
  let col = seatid & 7; // col is the last 3 bits
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
