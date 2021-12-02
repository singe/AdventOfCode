use std::io::BufRead;

fn main() {
  let mut x = 0; let mut y = 0; let mut z = 0;
  std::io::stdin().lock().lines()
    .for_each(|line| {
      let dir = line.as_ref().unwrap().chars().next().unwrap();
      let val = line.as_ref().unwrap().split(" ").nth(1).unwrap().parse::<usize>().unwrap();
      match dir {
        'f' => { 
                 x += val;
                 y += z*val;
               },
        'd' => z += val,
        'u' => z -= val,
        _ => (),
      }
    });
  println!("x:{} y:{} z:{} mul:{}",x,y,z,x*y);
}
