use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &String) -> Vec<usize> {
  let mut db: Vec<usize> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        //println!("Hello World {:?}", result);
        db.push(result.parse().unwrap());
      }
    }
  }
  return db;
}

fn check(db: &Vec<usize>, preamlen: usize) -> Result<i32,i32>
{
  if preamlen < 2 || preamlen >= db.len() {
    return Err(-1)
  }
  let mut valid: i32 = -1;
  for window in db.windows(preamlen+1) {
    let mut found = false;
    //check last num is list against preamble
    let num = window.last().unwrap();
    let mut x = 0;

    while x < window.len()-2 && !found {
      if window[x] >= *num {
        x += 1;
        continue;
      }
      //println!("num {} check {} window {:?}",num,num-window[x],window);
      found = window.contains(&(num-window[x]));
      x += 1;
    }
    if !found {
      println!("Bad {}",num);
      valid = *num as i32;
    } else {
      println!("Good {}",num);
    }
  }
  if valid != -1 {
    return Err(valid);
  }
  return Ok(0);
}

fn find_chain(db: &Vec<usize>, badnum: usize) -> Vec<usize>
{
  let mut sum = 0;
  let mut chain: Vec<usize> = vec![];
  let mut i = db.len()-1;
  //for elem in db.iter().rev() { // can't do this, need to rewind
  // while i >= 0 { // lol, rust pointed out that's the definition of usize
  loop {
    let elem = db[i];
    println!("elem {} sum {}",elem,sum);
    // too big skip
    if elem >= badnum { i -= 1; continue; }
    match elem + sum {
      e if e == badnum => {
                            // we've done it!
                            chain.push(elem);
                            chain.sort();
                            return chain;
                          },
      e if e < badnum => {
                          // looking good keep summing
                          chain.push(elem);
                          sum += elem;
                         },
      e if e > badnum => {
                          // doh rewind the loop and clear the chain
                          i += chain.len();
                          chain.clear();
                          sum = 0;
                         },
      _ => (),
    }
    // we're going backwards through the loop and usize can be negative
    if i == 1 {
      break;
    } else {
      i -= 1;
    }
  }
  return chain;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let preamlen = &args[2].parse().unwrap();
  if let Err(bad) = check(&db,*preamlen) {
    let chain = find_chain(&db,bad as usize);
    let answer = chain.first().unwrap() + chain.last().unwrap();
    println!("bad number {} chain {:?} answer {}",bad,chain,answer);
  }
}
