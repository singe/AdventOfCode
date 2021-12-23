use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct WIRE {
  reading: Vec<HashSet<char>>,
  number: Vec<HashSet<char>>,
}

fn parse(filename: &String) -> Vec<WIRE> {
  let mut db: Vec<WIRE> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        let maps = result.split(" | ").nth(0).unwrap();
        let words = maps.split(" ").collect::<Vec<&str>>();
        let reads: Vec<HashSet<char>> = words.iter().map(|s| s.to_string().chars().collect::<HashSet<char>>()).collect();

        let maps = result.split(" | ").nth(1).unwrap();
        let words = maps.split(" ").collect::<Vec<&str>>();
        let nums: Vec<HashSet<char>> = words.iter().map(|s| s.to_string().chars().collect::<HashSet<char>>()).collect();
        //let words = nums.split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
        let wire: WIRE = WIRE {
          reading: reads,
          number: nums,
        };
        db.push(wire);
      }
    }
  }
  return db;
}

fn check(db: &Vec<WIRE>) 
{
  let mut grand_total = 0;
  for entry in db {
    let mut lengths: HashMap<usize,Vec<&HashSet<char>>> = HashMap::new();
    for i in 2..=7 {
      lengths.insert(i,vec![]);
    }

    for read in &entry.reading {
      for i in 0..=7 {
        if read.len() == i { 
          lengths.get_mut(&i).unwrap().push(read);
        }
      }
    }
    let one = lengths[&2][0];
    let four = lengths[&4][0];
    let seven = lengths[&3][0];
    let eight = lengths[&7][0];

    let mut six: HashSet<_> = HashSet::new();
    for i in 0..lengths[&6].len() {
      let x = lengths[&6][i];
      if ! x.is_superset(one) {
        six = x.clone();
        lengths.get_mut(&6).unwrap().swap_remove(i);
        break;
      }
    }
    let mut five: HashSet<_> = HashSet::new();
    for i in 0..lengths[&5].len() {
      let x = lengths[&5][i];
      if x.is_subset(&six) {
        five = x.clone();
        lengths.get_mut(&5).unwrap().swap_remove(i);
        break;
      }
    }
    let mut nine: HashSet<_> = HashSet::new();
    for i in 0..lengths[&6].len() {
      let x = lengths[&6][i];
      if x.is_superset(&five) {
        nine = x.clone();
        lengths.get_mut(&6).unwrap().swap_remove(i);
        break;
      }
    }
    let mut three: HashSet<_> = HashSet::new();
    for i in 0..lengths[&5].len() {
      let x = lengths[&5][i];
      if x.is_subset(&nine) {
        three = x.clone();
        lengths.get_mut(&5).unwrap().swap_remove(i);
        break;
      }
    }
    let zero = lengths[&6][0];
    let two = lengths[&5][0];
    // we have all the numbers
    let mapping = [zero, one, two, &three, four, &five, &six, seven, eight, &nine];

    let mut total = 0; 
    entry.number.iter().enumerate().for_each(|(i,x)| {
      //println!("{} {}",i,10_u32.pow((3-i) as u32));
      for j in 0..mapping.len() {
        if x == mapping[j] { total += 10_u32.pow((3-i) as u32)*(j as u32) }
      }
    });
    println!("{}",total);
    grand_total += total;

    // clear for the next round
    lengths.clear();
  }
    println!("{}",grand_total);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  check(&db);
}
