use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// Open the input file and parse the two columns of numbers into two arrays
pub fn parse<P: AsRef<Path>>(path: P) -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in reader.lines() {
        let line = line?;
        // Split the line on the whitespace between the two columns
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            if let (Ok(num1), Ok(num2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                vec1.push(num1);
                vec2.push(num2);
            } else {
                eprintln!("[x] Couldn't convert some of the numbers to an integer: {}", line);
            }
        } else {
            eprintln!("[x] Line is missing two columns: {}", line);
        }
    }

    Ok((vec1, vec2))
}

// Calculate the total difference between each sorted vector's relative position 
pub fn diff(vec1: &Vec<i32>, vec2: &Vec<i32>) -> i32 {
    let mut sorted1 = vec1.clone();
    let mut sorted2 = vec2.clone();
    sorted1.sort_unstable();
    sorted2.sort_unstable();

    // They need to be the same length for this to work
    assert_eq!(sorted1.len(),sorted2.len());

    (0..sorted1.len())
        .map(|i| (sorted1[i] - sorted2[i]).abs())
        .sum()
}

use std::collections::HashMap;

// Count the number of times a number shows up in the list and produce a hashmap
// we can lookup against
pub fn frequency(numbers: &Vec<i32>) -> HashMap<i32, usize> {
    let mut freq_map = HashMap::new();

    for num in numbers {
        *freq_map.entry(*num).or_insert(0) += 1;
    }

    freq_map
}

// Check the frequency of each number in the Vec and multiply the freq with the
// element. Then add those up.
pub fn frequency_sum(numbers: &Vec<i32>, freq_map: &HashMap<i32, usize>) -> i32 {
    numbers
        .iter()
        .filter_map(|&num| freq_map.get(&num).map(|&freq| num * freq as i32))
        .sum()
}