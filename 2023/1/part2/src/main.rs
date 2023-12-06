use aho_corasick::AhoCorasick;
use std::io::BufRead;

fn main() {
    let words = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2",
        "3", "4", "5", "6", "7", "8", "9",
    ];
    let ac = AhoCorasick::new(&words);

    let numbers: Vec<usize> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| {
            let line = l.expect("Can't read line");

            let mut first_match = None;
            let mut last_match = None;

            for i in 0..line.len() {
                for matched in ac.find_iter(&line[i..]) {
                    if first_match == None {
                        first_match = Some(matched);
                        last_match = first_match.clone();
                    } else {
                        last_match = Some(matched);
                    }
                    let global_start = last_match.clone().unwrap().start() + i;
                }
            }

            let mut first = 0;
            let mut last = 0;
            if first_match == None || last_match == None {
                println!("We didn't extract a number {first_match:?} {last_match:?} from {line}");
                std::process::exit(1);
            } else {
                first = first_match.unwrap().pattern();
                last = last_match.unwrap().pattern();
                //println!("first {} last {}\t{line}", &words[first], &words[last]);
            }

            first = if first < 9 { first + 1 } else { first - 8 };
            last = if last < 9 { last + 1 } else { last - 8 };
            let result = (first * 10) + last;
            println!("{result}");
            result
        })
        .collect();

    let total: usize = numbers.iter().sum();
    println!("Result: {total}");
}
