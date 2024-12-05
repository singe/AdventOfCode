use one::{parse, diff, frequency, frequency_sum};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <path_to_file>", args[0]);
        return Err("[x] Invalid number of arguments".into());
    }

    let path = &args[1];

    match parse(path) {
        Ok((vec1, vec2)) => {
            let result_part1 = diff(&vec1, &vec2);
            println!("[+] Part1: The total distance is {}", result_part1);

            let frequency_table = frequency(&vec2);
            let result_part2 = frequency_sum(&vec1, &frequency_table);
            println!("[+] Part2: The total similarity is {}", result_part2);
        }
        Err(e) => eprintln!("[x] Error reading file: {}", e),
    }

    Ok(())
}