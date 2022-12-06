use std::io::BufRead;

fn main() {
    let mut lines = std::io::stdin().lock().lines().map(|l| l.unwrap());
    for input in lines {
        for (pos,block) in input.chars().collect::<Vec<char>>().windows(4).enumerate() {
            let mut found = true;
            for (i,c) in block.iter().enumerate() {
                if block[i+1..].contains(&c) { found = false; continue; }
            }
            if found { let end = pos + 4; println!("{end:?} {block:?}"); break; }
        }
    }
}
