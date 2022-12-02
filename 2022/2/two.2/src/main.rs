use std::io::BufRead;

fn main() {
    let score: usize = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(' ')
                .map(|x| match x {
                    "A" => 1,
                    "B" => 2,
                    "C" => 3,
                    "X" => 0,
                    "Y" => 3,
                    "Z" => 6,
                    _ => unreachable!(),
                })
                .collect::<Vec<usize>>()
        })
        .map(|y| {
            (match y[0] + y[1] {
                4 | 2 | 9 => 1,
                7 | 5 | 3 => 2,
                1 | 8 | 6 => 3,
                _ => unreachable!(),
            }) + y[1]
        })
        .sum();
    println!("{score:?}");
}
