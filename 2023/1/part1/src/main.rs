use std::io::BufRead;

fn main() {
    let mut first = ' ';
    let mut last = ' ';

    let numbers: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.expect("Can't read line");
            for c in line.chars() {
                if c.is_ascii_digit() {
                    first = c;
                }
            }

            for c in line.chars().rev() {
                if c.is_ascii_digit() {
                    last = c;
                }
            }

            if first == ' ' || last == ' ' {
                println!("We didn't extract a number, first is {first} and last is {last}");
                std::process::exit(1);
            }

            let str_number = format!("{last}{first}");
            match str_number.parse::<i32>() {
                Ok(num) => num,
                Err(e) => {
                    println!("Failed to parse: {}", e);
                    std::process::exit(2);
                }
            }
        })
        .collect();

    let total: i32 = numbers.iter().sum();
    println!("Result: {total}");
}
