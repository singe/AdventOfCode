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

fn ongrid(position: (i64,i64), width: (usize,usize)) -> Result<(usize, usize),(usize, usize)>
{
  //println!("in {:?}",position);
  let mut change = false;
  let x = match position.0 {
    -1 => { change=true; 0 },
    e if e > width.0 as i64 => { change=true; width.0 },
    _ => position.0 as usize,
  };

  let y = match position.1 {
    -1 => { change=true; 0 },
    e if e > width.1 as i64 => { change=true; width.1 },
    _ => position.1 as usize,
  };

  //println!("out {} {:?}",change,(x,y));
  if change {
    return Err((x,y));
  } else {
    return Ok((x,y));
  }
}

fn los(map: &MAP, start: (usize,usize), direction: (i8,i8)) -> char
{
  let (mut x, mut y) = start;
  loop {
    match ongrid((x as i64 + direction.0 as i64,y as i64 + direction.1 as i64), map.width) {
      Err(_) => break,
      Ok(a) => { x = a.0; y = a.1; },
    };
    if let Some(chr) = map.map.get(&(x,y)) {
      if chr == &'.' { continue; } // floor, keep looking
      else {
        return *chr;
      }
    }
  }
  return 'x';
}

// Check the eight adjacent positions
fn adjacent(map: &MAP, position: (usize,usize), check: char) -> usize
{
  let mut count = 0;
  for x in -1..=1 {
    for y in -1..=1 {
      if (x,y) == (0,0) { continue; }
      if los(&map, position, (x,y)) == check {
        count += 1;
      }
    }
  }
  //println!("found {}",count);
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
      //1 If a seat is empty (L) and there are no occupied seats in sight of
      //it, the seat becomes occupied.
      'L' => {
        if adjacent(&map, *key.0, '#') == 0 {
          newmap.occupied += 1;
          newmap.map.insert(*key.0,'#');
          changes += 1;
        }
      },
      //2 If a seat is occupied (#) and four or more seats adjacent to it are
      //also occupied, the seat becomes empty.
      //it now takes five or more visible occupied seats for an occupied seat
      //to become empty 
      '#' => {
        newmap.occupied += 1;
        if adjacent(&map, *key.0, '#') > 4 {
          newmap.occupied -= 1;
          newmap.map.insert(*key.0,'L');
          changes += 1;
        }
      },
      _ => (),
    }
  }
  //println!("changes {} map {:?}",changes,map);
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
