use std::io::BufRead;

fn main() {
  let mut x = 0; let mut y = 0;
  std::io::stdin().lock().lines()
    .for_each(|line| {
      let dir = line.as_ref().unwrap().chars().next().unwrap();
      let val = line.as_ref().unwrap().split(" ").nth(1).unwrap().parse::<usize>().unwrap();
      match dir {
        'f' => x += val,
        'd' => y += val,
        'u' => y -= val,
        _ => (),
      }
    });
  println!("x:{} y:{} mul:{}",x,y,x*y);
}
