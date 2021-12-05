use std::io::BufRead;

fn main() {
  let mut counts: Vec<(u32,u32)> = vec![(0,0); 12];

  std::io::stdin().lock().lines()
    .for_each(|line| {
      line.as_ref().unwrap().chars().enumerate()
        .for_each(|(i, c)| {
          let chr = c.to_digit(2).unwrap();
          match chr {
            0 => counts[i].0 += 1,
            1 => counts[i].1 += 1,
            _ => (),
          }
        });
    });

  // bit shift our 1's and 0's to binary
  let mut gamma: u32 = 0; let mut epsilon: u32 = 0;
  counts.iter().enumerate()
    .for_each(|(i, n)| {
      //println!("{} {:?}",i,n);
      let one = 1 << counts.len()-1-i;
      if n.1 > n.0 {
        gamma ^= one;
      } else {
        epsilon ^= one;
      }
    });
  println!("counts:{:?} gamma:{:?} epsilon:{:?} mul:{}",counts,gamma,epsilon,gamma*epsilon);
}
