use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<Vec<u8>> {
  let mut db: Vec<Vec<u8>> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        db.push(result.chars().map(|c| c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>());
      }
    }
  }
  return db;
}

fn check(db: &Vec<Vec<u8>>) 
{
  let mut lows: Vec<[u8;3]> = Vec::new();
  let mut risks: Vec<usize> = Vec::new();
  for y in 0..db.len() {
    for x in 0..db[y].len() {
      println!("pos {},{}",x,y);
      let mut brk = false;
      let mut checked: Vec<(usize,usize)> = Vec::new();
      // do a circle with i and j
      for (i,j) in [(-1,0),(1,0),(0,-1),(0,1)] {
        // make sure the result stays in bounds
        let xind = match (x as i32 + i) {
         e if e < 0 => 0 as usize,
         e if e == db[y].len() as i32 => x,
         e => e as usize,
        };
        let yind = match (y as i32 + j) {
         e if e < 0 => 0 as usize,
         e if e == db.len() as i32 => y,
         e => e as usize,
        };
        println!("  i {},j {}  ind {},{}",i,j,xind,yind);
        // avoid checking positions we've checked, or our current pos
        if checked.contains(&(xind,yind)) || ((xind,yind) == (x,y)) { println!("  seen before"); continue; }
        checked.push((xind,yind));
        println!("  cmp {},{} = {} to {},{} = {}",x,y,db[y][x],xind,yind,db[yind][xind]);
        if db[yind][xind] <= db[y][x] {
          println!("  not lowest, bailing");
          brk = true;
          break;
        }
      }
      if !brk { 
        println!("  found {} at {},{}",db[y][x],x,y);
        lows.push([db[y][x],x as u8,y as u8]); 
        risks.push(1 + db[y][x] as usize);
      }
    }
  }
  println!("{:?}",lows);
  println!("{:?}",risks);
  println!("result {}",risks.iter().sum::<usize>());
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
