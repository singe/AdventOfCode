use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> (HashMap<(usize,usize),char>,(usize,usize)) {
  let mut map = HashMap::new();
  let (mut x, mut y) = (0, 0);
  let mut width = (0, 0);
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        for chr in result.chars() {
          map.insert((x,y),chr);
          width.0 = x;
          x = x + 1;
        }
        width.1 = y;
        y = y + 1;
        x = 0;
      }
    }
  }
  return (map, width);
}

fn parse_instructions(instructions: &String) -> (usize, usize) {
  let (mut x, mut y) = (0, 0);
  for chr in instructions.chars().collect::<Vec<char>>().chunks(2) {
    let num: usize = chr[1].to_string().parse().unwrap();
    match chr[0] {
      'r' => x = x + num,
      'l' => x = x - num,
      'd' => y = y + num,
      'u' => y = y - num,
      _ => (),
    }
  }
  return (x, y);
}

fn route(map: &HashMap<(usize,usize),char>, width: (usize,usize), movement: (usize, usize)) -> usize
{
  let (mut x, mut y) = movement;
  let mut trees = 0;

  loop {
    x = x + movement.0;
    x = x % (width.0+1);
    y = y + movement.1;
    if x <= width.0 && y <= width.1 {
      //println!("{}",map.get(&(x,y)).unwrap());
      if *map.get(&(x,y)).unwrap() == '#' {
        trees = trees + 1;
      }
    } else {
      break;
    }
  }

  return trees;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let (map, width) = parse(&args[1]);
  let movement = parse_instructions(&args[2]);
  let trees = route(&map, width, movement);
  println!("You'd encounter {} trees.",trees.to_string());
}
