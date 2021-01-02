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

fn check(record: &String) -> (isize,isize,isize) 
{
  // It's binary just replace the chars with 1/0s and convert
  let rowstr = String::from(&record[0..7]);
  let colstr = String::from(&record[7..10]);
  let rowbinstr = rowstr.replace("F","0").replace("B","1");
  let colbinstr = colstr.replace("L","0").replace("R","1");
  let row = isize::from_str_radix(&rowbinstr, 2).unwrap_or(0);
  let col = isize::from_str_radix(&colbinstr, 2).unwrap_or(0);
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
