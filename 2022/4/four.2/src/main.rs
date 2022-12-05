use std::io::{stdin, BufRead};
use std::ops::RangeInclusive;


fn main() {
    let mut count = 0;

    let lines = stdin().lock().lines().map(|l| l.unwrap());
    for line in lines {
        let mut pair = line.split(',');
        let mut one = pair.next().unwrap().split('-');
        let mut two = pair.next().unwrap().split('-');
        let range1: RangeInclusive<usize> =
            one.next().unwrap().parse().unwrap()..=one.next().unwrap().parse().unwrap();
        let range2: RangeInclusive<usize> =
            two.next().unwrap().parse().unwrap()..=two.next().unwrap().parse().unwrap();

        for i in range1 {
            if range2.contains(&i) { count += 1; break; }
        }
    }
    println!("{count}");
}
