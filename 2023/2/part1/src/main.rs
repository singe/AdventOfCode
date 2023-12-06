use maplit::hashmap;
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
        .expect("Unsuccessful parse").next().unwrap();

    let base: HashMap<String, u32> = hashmap! {
        "red".to_string() => 12,
        "green".to_string() => 13,
        "blue".to_string() => 14,
    };

/* Before trying with iterator comprehensions
    let mut sum = 0;
    for (id, game) in file.into_inner().enumerate() {
        let mut possible = true;
        if let Rule::game = game.as_rule() {
            for set in game.into_inner().skip(1) {
                for pair in set.into_inner() {
                    let mut nums_colours = pair.into_inner();
                    let number = nums_colours
                        .next().unwrap().as_str().parse::<u32>().unwrap();
                    let colour = nums_colours.next().unwrap().as_str().to_string();
                    if number > *base.get(&colour).expect("Couldn't find colour {colour}") {
                        println!("Removing {id} because {} for {}", number, colour);
                        possible = false;
                        break;
                    }
                }
                if !possible {
                    break;
                }
            }
            if possible {
                sum += id + 1;
            }
        }
    }
    println!("{sum}");
*/
    let sum = file
        .into_inner()
        .enumerate()
        .filter_map(|(id, game)| {
            if let Rule::game = game.as_rule() {
                let possible = game.into_inner().skip(1).all(|set| {
                    set.into_inner().all(|pair| {
                        let mut nums_colours = pair.into_inner();
                        let number = nums_colours
                            .next().unwrap().as_str().parse::<u32>().unwrap();
                        let colour = nums_colours.next().unwrap().as_str().to_string();
                        number <= *base.get(&colour).unwrap_or(&0)
                    })
                });
                if possible {
                    Some(id + 1) // Game IDs are 1-indexed
                } else {
                    None
                }
            } else {
                None
            }
        })
        .sum::<usize>();
    println!("{sum}");
}
