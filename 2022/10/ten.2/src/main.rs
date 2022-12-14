use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut x: i32 = 1;
    let mut cycle = 0;

    let mut screen = vec![vec![0; 40]; 6];

    for line in lines {
        let mut cycle_increment = 0;
        let mut x_increment = 0;
        let mut instructions = line.split(' ');
        match instructions.next() {
            Some("noop") => {
                cycle_increment += 1;
            }
            Some("addx") => {
                let value = instructions.next().unwrap().parse::<i32>().unwrap();
                x_increment = value;
                cycle_increment += 2;
            }
            _ => (),
        }

        for i in cycle + 1..=cycle + cycle_increment {
            cycle = i;
            let row = (cycle - 1) % 40;
            let col = (cycle - 1) / 40;
            if (x - 1..=x + 1).contains(&row) {
                let h: usize = col.try_into().unwrap();
                let v: usize = row.try_into().unwrap();
                screen[h][v] = 1;
            }
        }
        x += x_increment;
    }
    for h in screen {
        for v in h {
            match v {
                1 => print!("#"),
                _ => print!("."),
            }
        }
        println!();
    }
}
