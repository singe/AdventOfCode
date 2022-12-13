use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines().map(|l| l.unwrap());
    let mut map: Vec<Vec<usize>> = vec![vec![0;1000];1000];

    let start_pos = (500,500);
    let mut head_pos = start_pos;
    let mut tail_pos = start_pos;
    map[tail_pos.1][tail_pos.0] += 1;
 
    for line in lines {
        println!("{line}");
        let mut instruction = line.split(' ');
        let dir = instruction.next().unwrap();
        let num = instruction.next().unwrap().parse::<usize>().unwrap();

        for i in 0..num {
            let prev_head = head_pos;
            match dir {
                "R" => {
                    head_pos.0 += 1;
                }
                "L" => {
                    head_pos.0 -= 1;
                },
                "U" => {
                    head_pos.1 -= 1;
                },
                "D" => {
                    head_pos.1 += 1;
                },
                _ => unreachable!(),
            }
            println!("H {head_pos:?}");

            let mut x_distance = 0;
            if head_pos.0 > tail_pos.0 {
                x_distance = head_pos.0 - tail_pos.0;
            } else {
                x_distance = tail_pos.0 - head_pos.0;
            }
            let mut y_distance = 0;
            if head_pos.1 > tail_pos.1 {
                y_distance = head_pos.1 - tail_pos.1;
            } else {
                y_distance = tail_pos.1 - head_pos.1;
            }
            if x_distance > 1 || y_distance > 1 {
                tail_pos = prev_head;
            }
            println!("T {tail_pos:?}");
            map[tail_pos.1][tail_pos.0] += 1;
        }
    }     
    let mut visited = 0;
    for y in map {
        for x in y {
            if x > 0 { visited += 1; }
        }
    }
    println!("{visited}");
}
