use std::io::BufRead;
use std::collections::VecDeque;

struct MONKEY {
    items: VecDeque<usize>,
    operation: Box<dyn Fn(usize) -> usize>,
    test: Box<dyn Fn(usize) -> bool>,
    divisor: usize,
    peers: (usize, usize),
    inspections: usize,
}

fn main() {
    let lines = std::io::stdin().lock().lines().map(|l| l.unwrap());

    let mut monkeys: Vec<MONKEY> = Vec::new();
    let mut items: VecDeque<usize> = VecDeque::new();
    let mut operation: Box<dyn Fn(usize) -> usize> = Box::new(|x| x + 1);
    let mut test: Box<dyn Fn(usize) -> bool> = Box::new(|_| true);
    let mut divisor = 0;
    let mut peers = (0,0);

    for line in lines {
        if line == "" { 
            monkeys.push( MONKEY {
                items: items.clone(),
                operation: operation,
                test: test,
                divisor: divisor,
                peers: peers,
                inspections: 0,
            }); 
            items.clear();
            operation = Box::new(|x| x + 1);
            test = Box::new(|_| true);
            divisor = 0;
            peers = (0,0);
            continue;
        }
        let mut tokens = line.split(' ');
        match tokens.next() {
            Some("Monkey") => {
                continue;
            },
            _ => { tokens.next(); },
        }
        match tokens.next() {
            Some("Starting") => {
                tokens.next();
                for x in tokens {
                    items.push_back(x.split(',').next().unwrap().parse::<usize>().unwrap());
                } 
                continue;
            },
            Some("Operation:") => {
                tokens.next(); tokens.next(); tokens.next();
                let op = tokens.next();
                let num = tokens.next();
                match op {
                    Some("+") => 
                        match num.unwrap().parse::<usize>() {
                            Ok(num) => operation = Box::new(move |old| old + num),
                            Err(_) => operation = Box::new(|old| old + old),
                        },
                    Some("*") => 
                        match num.unwrap().parse::<usize>() {
                            Ok(num) => operation = Box::new(move |old| old * num),
                            Err(_) => operation = Box::new(|old| old * old),
                        },
                    _ => unreachable!(),
                }
                continue;
            },
            Some("Test:") => {
                tokens.next(); tokens.next();
                divisor = tokens.next().unwrap().parse::<usize>().unwrap();
                test = Box::new(move |x| x % divisor == 0);
                continue;
            },
            _ => { tokens.next(); tokens.next(); }
        }
        match tokens.next() {
            Some("true:") => {
                tokens.next(); tokens.next(); tokens.next();
                peers.0 = tokens.next().unwrap().parse::<usize>().unwrap();
                continue;
            },
            Some("false:") => {
                tokens.next(); tokens.next(); tokens.next();
                peers.1 = tokens.next().unwrap().parse::<usize>().unwrap();
                continue;
            },
            _ => unreachable!(),
        }
    }
    monkeys.push( MONKEY {
        items: items.clone(),
        operation: operation,
        test: test,
        divisor: divisor,
        peers: peers,
        inspections: 0,
    }); 

    let mut supermod = monkeys[0].divisor * monkeys[1].divisor;
    for x in monkeys.iter().skip(2) {
       supermod = supermod * x.divisor;
    };

    for a in 0..10000 {
        println!("Round {a}");
        for i in 0..monkeys.len() {
            println!("  Monkey {i}");
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items[0];
                println!("    Item {item}");
                monkeys[i].inspections += 1;
                let worry = (monkeys[i].operation)(item) % supermod;
                println!("      Worry {worry}");
                let peers = monkeys[i].peers;
                monkeys[i].items.pop_front();
                if (monkeys[i].test)(worry) {
                    println!("     Passed {}",peers.0);
                    monkeys[peers.0].items.push_back(worry);
                } else {
                    println!("     Failed {}",peers.1);
                    monkeys[peers.1].items.push_back(worry);
                }
            }
        }
    }

    let mut inspects: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    println!("{inspects:?}");
    inspects.sort();
    let one = inspects.pop().unwrap();
    let two = inspects.pop().unwrap();
    let answer = one * two;
    println!("{answer}");
}
