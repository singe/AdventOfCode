use std::io::BufRead;

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut x = 1;
    let mut cycle = 0;
    let mut sample = 20;
    let mut result = 0;

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
            println!("{cycle} X:{x}");
            if cycle == sample {
                let strength = x * cycle;
                result += strength;
                println!("Strength: {strength} Cycle {cycle} X {x} Sample {sample}");
                sample += 40;
            }
        }
        x += x_increment;
    }
    println!("{result}");
}
