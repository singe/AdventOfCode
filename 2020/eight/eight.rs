use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct ASM {
  cmd: String,
  num: i32,
  nex: i32, //number of times executed
}

fn parse(filename: &String) -> Vec<ASM> {
  let mut db: Vec<ASM> = Vec::new();
  if let Ok(lines) = read_lines(filename) {
    for line in lines {
      if let Ok(result) = line {
        // e.g. jmp +2
        let mut splt = result.split(' ');
        let cmd = splt.next().unwrap_or("nop"); //jmp
        let val = splt.next().unwrap_or("+0"); //+2
        let mut num = val.split(&['+','-'][..]).nth(1).unwrap_or("0").parse().unwrap_or(0);
        match val.chars().next().unwrap() {
          //'+' => {},
          '-' => num *= -1,
          _ => {},
        }
        let asm: ASM = ASM {
          cmd: cmd.to_string(),
          num: num,
          nex: 0
        };
        //println!("input {:?}",asm);
        db.push(asm);
      }
    }
  }
  return db;
}

fn exec(db: &mut Vec<ASM>) -> bool 
{
  let mut acc: i32 = 0;

  let mut sp = 0;
  while sp < db.len() {
    let mut op = &mut db[sp];
    println!("exec {:?}",op); 
    if op.nex > 0 {
      println!("loop acc {}",acc);
      return false;
    }
    match &op.cmd[..] {
      "acc" => acc += op.num,
      "jmp" => {
                if op.num.is_negative() { sp -= op.num.wrapping_abs() as usize }
                else { sp += op.num as usize }
                sp -= 1;
               },
      _ => {},
    }
    op.nex += 1;
    println!("acc {}",acc);
    sp += 1;
  }
  return true;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let mut db = parse(&args[1]);
  exec(&mut db);
}
