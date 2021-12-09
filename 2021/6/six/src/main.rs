use std::io::{BufRead};

fn main() {
  let mut db: Vec<u8> = Vec::new();
  std::io::stdin().lock().lines()
    .for_each(|line| line.as_ref().unwrap().split(",")
      .for_each(|x| db.push(x.parse::<u8>().unwrap())));

  let days = 18;
  println!("Initial state: {:?}",db);
  for day in 1..=days {
    for i in 0..db.len() {
      match db[i] {
        0 => { db.push(8); db[i] = 6; },
        _ => db[i] -= 1,
      };
    }
    println!("After {:02} days: {:?}",day,db);
  }
  println!("{} fish",db.len());
}
