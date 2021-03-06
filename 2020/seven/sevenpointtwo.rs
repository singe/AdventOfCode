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

fn parse(filename: &String) -> HashMap<String,HashMap<String,usize>> {
  let mut db: HashMap<String,HashMap<String,usize>> = HashMap::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        // light red bags contain 1 bright white bag, 2 muted yellow bags.
        let mut splt = result.split(" contain ");
        let keystr = splt.next().unwrap();
        let valuestr = splt.next().unwrap();
        //key == "light red bags"
        //value = "3 bright white bags, 4 muted yellow bags." or "1 shiny gold bag." or "no other bags."

        //key "light red"
        let mut tmp = keystr.split(' ');
        let key = format!("{} {}",tmp.next().unwrap(),tmp.next().unwrap());
        if db.get(&key) == None {
          db.insert(key.clone(),HashMap::new());
        }
        //value
        for bag in valuestr.split(", ") {
          let mut splt = bag.split(' ');
          let num: usize = splt.next().unwrap_or("0").parse().unwrap_or(0);
          if num == 0 { break; } // "no other bags"
          let bag = format!("{} {}",splt.next().unwrap(),splt.next().unwrap());
          db.get_mut(&key).unwrap().insert(bag,num);
        }
      }
    }
  }
  return db;
}

fn find_rules(db: &HashMap<String,HashMap<String,usize>>, bag: &str) -> usize
{
  let mut matches = 0;

  if let Some(bags) = db.get(bag) {
    if bags.len() == 0 { return matches; }
    for record in bags {
      matches += record.1 + ( record.1 * find_rules(&db,&record.0) );
    }
  }

  return matches;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let db = parse(&args[1]);
  let rules = find_rules(&db,&"shiny gold");
  println!("total bags {}",rules);
}
