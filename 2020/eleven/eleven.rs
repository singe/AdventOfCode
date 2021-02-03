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

#[derive(Debug,Clone)]
struct MAP {
  map: HashMap<(usize,usize),char>,
  width: (usize, usize),
  occupied: usize,
}

fn parse(filename: &String) -> MAP
{
  let mut map: MAP = MAP {
    map: HashMap::new(),
    width: (0,0),
    occupied: 0,
  };
  read_lines(filename)
    .unwrap()
    .map(|n| n.unwrap())
    .enumerate()
    .for_each(|(y, line)| {
      line.chars().enumerate().for_each(|(x, chr)| {
        map.map.insert((x,y),chr);
        map.width = (x,y);
      });
    });
  return map;
}

// Check the eight adjacent positions
fn adjacent(map: &MAP, position: (usize,usize), check: char) -> usize
{
  // Make sure we're in range >0 and <width for x & y
  let minx = match position.0.checked_sub(1) {
    None => 0,
    Some(num) => num,
  };
  let miny = match position.1.checked_sub(1) {
    None => 0,
    Some(num) => num,
  };
  let maxx = match position.0 {
    e if e == map.width.0 => map.width.0,
    _ => position.0+1,
  };
  let maxy = match position.1 {
    e if e == map.width.1 => map.width.1,
    _ => position.1+1,
  };
  let mut count = 0;
  for x in minx..=maxx {
    for y in miny..=maxy {
      if (x,y) == position { continue; }
      if let Some(chr) = map.map.get(&(x,y)) {
        if chr == &check { count += 1; }
      }
    }
  }
  return count;
}

// Apply state change rules & count occupied seats
fn round(map: &MAP) -> (usize,MAP)
{
  let mut changes = 0;
  let mut newmap = map.clone(); //changes must interfere with old state
  newmap.occupied = 0; //reset occupied
  for key in &map.map {
    match key.1 {
      //1 If a seat is empty (L) and there are no occupied seats adjacent to
      //it, the seat becomes occupied.
      'L' => {
        if adjacent(&map, *key.0, '#') == 0 {
          newmap.map.insert(*key.0,'#');
          changes += 1;
        }
      },
      //2 If a seat is occupied (#) and four or more seats adjacent to it are
      //also occupied, the seat becomes empty.
      '#' => {
        newmap.occupied += 1;
        if adjacent(&map, *key.0, '#') > 3 {
          newmap.map.insert(*key.0,'L');
          changes += 1;
        }
      },
      _ => (),
    }
  }
  println!("changes {}",changes);
  return (changes,newmap);
}

// run until no more state changes
fn run(map: &MAP) -> (usize, MAP)
{
  let mut rounds = 0;
  let mut newmap: MAP = map.clone();
  loop {
    let (changes, tmpmap) = round(&newmap);
    if changes != 0 {
      rounds += 1;
      newmap = tmpmap;
    } else {
      break;
    }
  }
  return (rounds, newmap);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let map = parse(&args[1]);
  let (rounds, map) = run(&map);
  println!("rounds {} occupied {}",rounds,map.occupied);
}
