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
  let mut db: Vec<Vec<u8>> = vec![vec![0; 990]; 990];
  //let mut db: Vec<Vec<u8>> = vec![vec![0; 10]; 10];
  let mut count = 0;
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        let mut splt = result.split(" -> ");
        let mut start = splt.next().unwrap().split(",");
        let startx = start.next().unwrap().parse::<usize>().unwrap();
        let starty = start.next().unwrap().parse::<usize>().unwrap();
        let mut end = splt.next().unwrap().split(",");
        let endx = end.next().unwrap().parse::<usize>().unwrap();
        let endy = end.next().unwrap().parse::<usize>().unwrap();
        //println!("{},{} -> {},{}",startx,starty,endx,endy);
        if (startx == endx) || (starty == endy) {
          let mut x0 = startx; let mut x1 = endx; let mut y0 = starty; let mut y1 = endy;
          if startx > endx { x0 = endx; x1 = startx; }
          if starty > endy { y0 = endy; y1 = starty; }
          for x in x0..=x1 {
            for y in y0..=y1 {
              //println!("Marking {},{}",x,y);
              db[y][x] += 1;
              if db[y][x] == 2 { count += 1; }
            }
          }
        }
      }
    }
  }
  println!("{} intersections",count);
  return db;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let _db = parse(&args[1]);
}
