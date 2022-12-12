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

    let mut visible_count = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let height = map[y][x];

            if x == 0 || x == max_x-1 || y == 0 || y == max_y-1 {
                visible_count += 1;
                continue;
            }

            let mut found;

            found  = false;
            for check_x in 0..x {
               if map[y][check_x] >= height { found = true; break; }
            }
            if !found { visible_count += 1; continue; }

            found = false;
            for check_x in x+1..max_x {
               if map[y][check_x] >= height { found = true; break; }
            }
            if !found { visible_count += 1; continue; }

            found  = false;
            for check_y in 0..y {
               if map[check_y][x] >= height { found = true; break; }
            }
            if !found { visible_count += 1; continue; }

            found = false;
            for check_y in y+1..max_y {
               if map[check_y][x] >= height { found = true; break; }
            }
            if !found { visible_count += 1; continue; }

        }
    }
    println!("{visible_count:?}");
}
