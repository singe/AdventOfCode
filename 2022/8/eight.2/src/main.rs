use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let map = lines
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let max_y = map.len();
    let max_x = map[0].len();

    let mut highest_score = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let height = map[y][x];

            if x == 0 || x == max_x-1 || y == 0 || y == max_y-1 {
                continue;
            }

            let mut left = 0;
            for check_x in (0..x).rev() {
               left += 1; 
               if map[y][check_x] >= height { break; }
            }

            let mut right = 0;
            for check_x in x+1..max_x {
               right += 1;
               if map[y][check_x] >= height { break; }
            }

            let mut up = 0;
            for check_y in (0..y).rev() {
               up += 1;
               if map[check_y][x] >= height { break; }
            }

            let mut down = 0;
            for check_y in y+1..max_y {
               down += 1;
               if map[check_y][x] >= height { break; }
            }

            let score = up * down * right * left;
            if score > highest_score { highest_score = score; }
        }
    }
    println!("{highest_score:?}");
}
