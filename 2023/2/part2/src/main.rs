use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;
use std::env::args;
use std::fs::read_to_string;
#[derive(Parser)]
#[grammar = "grammar.pest"]
struct GameParser;

fn main() {
    let args: Vec<String> = args().collect();
    let path: &String = &args[1];
    let input = read_to_string(path).expect("Should have been able to read the file");
    let file = GameParser::parse(Rule::game_list, &input)
        .expect("Unsuccessful parse")
        .next()
        .unwrap();

    let mut base = HashMap::new();
    base.insert("red", 0);
    base.insert("green", 0);
    base.insert("blue", 0);

    let sum = file
        .into_inner()
        .filter_map(|game| {
            if let Rule::game = game.as_rule() {
                let mut largest = base.clone();
                game.into_inner().skip(1).for_each(|set| {
                    set.into_inner().for_each(|pair| {
                        let mut nums_colours = pair.into_inner();
                        let number = nums_colours
                            .next()
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap();
                        let colour = nums_colours.next().unwrap().as_str();
                        if number > *largest.get(colour).unwrap_or(&0) {
                            largest.insert(colour, number);
                        }
                    });
                });
                let power = largest.values().product::<u32>();
                Some(power)
            } else {
                None
            }
        })
        .sum::<u32>();
    println!("{sum}");
}
