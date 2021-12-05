use std::io::BufRead;

fn main() {
  let mut input = Vec::new();
  let mut tmp = Vec::new();
  std::io::stdin().lock().lines()
    .for_each(|line| {
      line.as_ref().unwrap().chars()
          .for_each(|c| {
            tmp.push(c.to_digit(2).unwrap());
          });
      input.push(tmp.clone());
      tmp.clear();
    });

  //input.iter().for_each(|x| println!("{:?}",x));
  let width = input[0].len();

  let mut o2 = input.clone();
  let mut co2 = input.clone();

  for i in 0..width {
    //o2.iter().for_each(|x| println!("{:?}",x));
    let mut count = 0;
    // count the number of 1's
    for j in 0..o2.len() {
      count += o2[j][i];
    }
    // what's half plus the remainder
    let half = ((o2.len()/2) + (o2.len()%2)) as u32;
    // do we have more 1's than half or 0's
    let verdict = match count {
      _ if count >= half => 1,
      _ => 0,
    };
    //println!("Verdict for width {} is {} from a count {} and half {}",i,verdict,count,half);
    // manual index, because we'll be removing elements
    let mut j = 0;
    loop {
      if j >= o2.len() { break; } // exit if we're at the end
      if o2.len() == 1 { break; } // only one left, it's the result
      if o2[j][i] != verdict { // look for elements that don't match
        //println!("Removing {:?} because pos {} doesn't match {}",o2[j],i,verdict);
        o2.remove(j); // delete it from the vector
        // no need to increment j, since we just moved the next one in from the left
      } else {
        j += 1; // it passed, no delete, increment j to see the next element
      }
    }
  }

  // I'm too lazy to figure out how to combine these right now
  // just repeat it for co2 but switch the verdict check
  for i in 0..width {
    //co2.iter().for_each(|x| println!("{:?}",x));
    let mut count = 0;
    for j in 0..co2.len() {
      count += co2[j][i];
    }
    let half = ((co2.len()/2) + (co2.len()%2)) as u32;
    let verdict = match count {
      _ if count >= half => 0,
      _ => 1,
    };
    //println!("Verdict for width {} is {} from a count {} and half {}",i,verdict,count,half);
    let mut j = 0;
    loop {
      if j >= co2.len() { break; }
      if co2.len() == 1 { break; }
      if co2[j][i] != verdict {
        //println!("Removing {:?} because pos {} doesn't match {}",co2[j],i,verdict);
        co2.remove(j);
      } else {
        j += 1;
      }
    }
  }

  // Convert the resulting array to binary with some bit shifting
  let mut o2dec = 0;
  o2[0].iter().enumerate().for_each(|(i, x)| {
    o2dec ^= x << width-1-i;
  });
  let mut co2dec = 0;
  co2[0].iter().enumerate().for_each(|(i, x)| {
    co2dec ^= x << width-1-i;
  });
  //println!("{:b} {}",o2dec,o2dec);
  //println!("{:b} {}",co2dec,co2dec);
  println!("Oxygen Generator rating {}, CO2 Scrubber rating {}, Multiplied {}",o2dec,co2dec,o2dec*co2dec);
}
