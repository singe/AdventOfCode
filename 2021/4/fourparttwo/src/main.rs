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
#[derive(Clone)]
struct BOARD {
  won: bool,
  width: usize,
  marks: usize, 
  // kleep track of wins in a row/col to avoid expensive traversals
  row_marks: Vec<usize>,
  col_marks: Vec<usize>,
  // 2D array for the board
  // each position has a tuple
  // with a bool to mark it
  board: Vec<Vec<(usize,bool)>>,
}

fn parse(filename: &String) -> (Vec<usize>,Vec<BOARD>) {
  let mut db: Vec<BOARD> = Vec::new();
  let mut numbers: Vec<usize> = Vec::new();
  let width = 5;
  let mut board: BOARD = BOARD {
    won: false,
    width: width,
    marks: 0,
    row_marks: vec![0; width],
    col_marks: vec![0; width],
    board: vec![vec![(0,false); width]; width],
  };
  let mut row = 0;
  read_lines(filename).unwrap().enumerate()
    .for_each(|(i, line)| {

      if i == 0 { 
        // parse first line of numbers
        numbers.append(
          &mut line.unwrap()
                 .split(',')
                 .map(|x| x.parse::<usize>().unwrap())
                 .collect::<Vec<usize>>());
      } else {
        // parse boards, skip blank line
        if line.as_ref().unwrap().len() != 0 {
          // add board row to current board
          line.unwrap()
            .split(' ')
            // filter out empty elements from double spaces
            .filter(|x| x.trim().len() > 0)
            .enumerate()
            .for_each(|(i,x)| {
              board.board[row][i].0 = x.trim().parse::<usize>().unwrap();
            });
          row += 1;
          if row == width { 
            //board.board.iter().for_each(|x| println!("{} {:?}",i,x));
            // end of board, store it and reset
            db.push(board.clone());
            board.board = vec![vec![(0,false); width]; width];
            row = 0;
          }
        }
      }

    });
  return (numbers,db);
}

fn check(db: &mut Vec<BOARD>, numbers: &Vec<usize>)
{
  let mut wins = 0;
  let length = db.len();
  for number in numbers {
    for board in &mut *db {
      for x in 0..board.width {
        for y in 0..board.width {
          if *number == board.board[x][y].0 {
            board.board[x][y].1 = true;
            board.marks += 1;
            board.row_marks[x] += 1;
            board.col_marks[y] += 1;
            if board.row_marks[x] == 5 || board.col_marks[y] == 5 {
              // only mark a board as won once
              if board.won == false {
                board.won = true;
                wins += 1;
              }
              // is this the last board to win
              if wins == length {
                //println!("winning {:?}",board);
                let mut unmark_sum = 0;
                board.board.iter().for_each(|row| { 
                  row.iter().for_each(|col| {
                    if col.1 { print!("x");
                    } else { 
                      print!("_");
                      unmark_sum += col.0;
                    }
                    print!("{:02} ",col.0);
                  });
                  println!("");
                });
                println!("Unmark sum {}, last num {}, mul {}",unmark_sum,number,unmark_sum*number);
                return;
              }
            }
          }
        }
      }
    }  
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let (numbers,mut db) = parse(&args[1]);
  check(&mut db,&numbers);
}
