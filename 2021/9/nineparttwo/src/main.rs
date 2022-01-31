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

//recursive function to find size of basin
fn basin(db: &Vec<Vec<u8>>, x: usize, y: usize, basins_seen: &mut Vec<(usize,usize)>) -> usize {
  let mut size = 0;
  for (i,j) in [(-1,0),(1,0),(0,-1),(0,1)] {
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
    println!("xy {},{} ij {},{} ind {},{}",x,y,i,j,xind,yind);
    if (db[yind][xind] != 9) && ((x,y) != (xind,yind) && !basins_seen.contains(&(xind,yind))) {
      println!("recurse {},{}",xind,yind);
      basins_seen.push((xind,yind));
      size += 1+basin(&db, xind, yind, basins_seen);
    }
  }
  return size;
}

//look for the low point
fn check(db: &Vec<Vec<u8>>) 
{
  let mut lows: Vec<[u8;3]> = Vec::new();
  let mut risks: Vec<usize> = Vec::new();
  let mut basins: Vec<usize> = Vec::new();
  let mut basins_seen: Vec<(usize,usize)> = Vec::new(); //prevent recheck in recursive
  for y in 0..db.len() {
    for x in 0..db[y].len() {
      println!("pos {},{}",x,y);
      let mut brk = false;
      let mut lowest_checked: Vec<(usize,usize)> = Vec::new(); //prevent rechecking
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
        if lowest_checked.contains(&(xind,yind)) || ((xind,yind) == (x,y)) { println!("  seen before"); continue; }
        lowest_checked.push((xind,yind));
        println!("  cmp {},{} = {} to {},{} = {}",x,y,db[y][x],xind,yind,db[yind][xind]);
        if db[yind][xind] <= db[y][x] {
          println!("  not lowest, bailing");
          brk = true;
          break;
        }
      }
      if !brk { 
        println!("  found {} at {},{}",db[y][x],x,y);
        basins.push(basin(&db,x,y,&mut basins_seen));
      }
    }
  }

  // get top three and multiply them together
  basins.sort();
  let len = basins.len();
  println!("{:?}",basins[len-1]*basins[len-2]*basins[len-3]);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
