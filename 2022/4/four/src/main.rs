use std::io::{stdin, BufRead};
use std::ops::Range;

fn is_subrange<T>(a: &Range<T>, b: &Range<T>) -> bool
where
    T: PartialOrd,
{
    a.start >= b.start && a.end <= b.end
}

fn main() {
    let mut count = 0;

    let lines = stdin().lock().lines().map(|l| l.unwrap());
    for line in lines {
        let mut pair = line.split(',');
        let mut one = pair.next().unwrap().split('-');
        let mut two = pair.next().unwrap().split('-');
        let range1: Range<usize> =
            one.next().unwrap().parse().unwrap()..one.next().unwrap().parse().unwrap();
        let range2: Range<usize> =
            two.next().unwrap().parse().unwrap()..two.next().unwrap().parse().unwrap();

        if is_subrange(&range1,&range2) || is_subrange(&range2,&range1){
            count += 1;
        }
    }
    println!("{count}");
}
