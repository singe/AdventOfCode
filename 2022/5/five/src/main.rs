use std::io::BufRead;
use regex::Regex;

fn main() {
    let mut lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut pos: Vec<(usize,String)> = Vec::new();
    let mut stack_count = 0;
    for line in lines.by_ref() {
        if line == "" { break; }
        for i in line.match_indices(char::is_uppercase) {
            let stack_num = i.0/4;
            if stack_num > stack_count { stack_count = stack_num; }
            pos.push((stack_num,i.1.to_string()));
        }
    }

    let mut stacks: Vec<Vec<String>> = vec![vec![];stack_count+1];
    for block in pos.iter().rev() {
        stacks[block.0].push(block.1.clone());
    }

    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").unwrap();
    for line in lines {
        for cap in re.captures_iter(&line) {
            let num = cap[1].parse::<usize>().unwrap();
            let src = cap[2].parse::<usize>().unwrap() - 1;
            let dst = cap[3].parse::<usize>().unwrap() - 1;

            for _ in 0..num {
                match stacks[src].pop() {
                    None => continue,
                    Some(b) => stacks[dst].push(b)
                }
            }
        }
    }

    for x in stacks {
        let y = x.last().unwrap();
        print!("{y}");
    }
    println!();
}
