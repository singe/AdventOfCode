use std::io::BufRead;

fn main() {
    let score: usize = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|x| match x {
                    "X" | "A" => 1,
                    "Y" | "B" => 2,
                    "Z" | "C" => 3,
                    _ => unreachable!(),
                })
                .collect::<Vec<usize>>()
        })
        .map(|y| {
            match y[1] as isize - y[0] as isize {
                 0 => 3 + y[1], //draw
                 1 => 6 + y[1], //win
                -2 => 6 + y[1], //win
                 2 => 0 + y[1], //loss
                -1 => 0 + y[1], //loss
                 _ => unreachable!(),
            }
        })
        .sum();
    println!("{score}");
}
