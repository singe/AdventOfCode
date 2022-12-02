use std::io::BufRead;

fn main() {
    let mut biggest = 0;
    std::io::stdin().lock()
        .lines()
        .map(
            |line| match line.expect("Line parse error").parse() {
                Ok(x) => x,
                Err(_) => 0,
            },
        )
        .reduce(|acc, item| {
            if acc > biggest {
                biggest = acc;
            }
            if item == 0 {
                0
            } else {
                acc + item
            }
        });
    println!("Total largest calories {biggest}");
}
