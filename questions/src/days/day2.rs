use std::{fs::File, io::Read};

use super::r#trait::Day;

pub struct Day2 { }

#[derive(Debug)]
struct Game {
    id: u32,
    mutations: Vec<Mutation>,
    valid: bool,
}

#[derive(Debug, Clone)]
struct Mutation {
    number: u32,
    color: String,
}

impl Day for Day2 {
    fn main(&self, file_name: &str) {
        let mut games = parse_file(file_name);

        // Rules for a valid game
        // Max 12 red
        // Max 13 green
        // Max 14 blue
        
        // What games are possible
        println!("Total games: {}", games.len());
        for game in &mut games {
            for mutation in &game.mutations {
                if mutation.color == "red" {
                    if mutation.number > 12 {
                        game.valid = false;
                    }
                } else if mutation.color == "green" {
                    if mutation.number > 13 {
                        game.valid = false;
                    }
                } else if mutation.color == "blue" {
                    if mutation.number > 14 {
                        game.valid = false;
                    }
                }
            }
        }

        // Sum valid game ids
        let sum = games.iter().fold(0, |a, b| {
            if b.valid {
               return a + b.id;
            }
            a
        });

        println!("Sum of valid game ids: {}", sum);


        // Part 2 - Find fewest
        let mut total_product_sum = 0;
        
        // Reduce games to 1 color and highest number
        for game in &mut games {
            let mut mutations: Vec<Mutation> = Vec::new();
            for mutation in &game.mutations {
                // Check if mutation is already in the list, if this one is larger, replace it
                let mut added =false;
                for (i, m) in mutations.iter().enumerate() {
                    if m.color == mutation.color {
                        added = true;
                        if m.number < mutation.number {
                            mutations[i] = mutation.clone();
                            break;
                        }
                    }
                }
                if !added {
                    mutations.push(mutation.clone());
                }
            }

            let mut product = 1;
            for m in mutations {
                product *= m.number;
            }
            total_product_sum += product;
        }

        println!("Total product sum: {}", total_product_sum);

    }
}

fn parse_file(file_name: &str) -> Vec<Game> {
    let mut result: Vec<Game> = Vec::new();

    // load File
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        // Get Game id
        let game_id = line.split_whitespace().nth(1).unwrap().trim_end_matches(":");
        let game = line.split(":").nth(1).unwrap();

        // Split each game into a vector of strings
        let games: Vec<&str> = game.split(";").collect();

        // Create a game
        let mut g = Game {
            id: game_id.parse::<u32>().unwrap(),
            mutations: Vec::new(),
            valid: true,
        };

        for game in games {
            let game_elements: Vec<&str> = game.split(",").collect();

            for inner_game in game_elements {
                g.mutations.push(Mutation {
                    number: inner_game.split_whitespace().nth(0).unwrap().parse::<u32>().unwrap(),
                    color: inner_game.split_whitespace().nth(1).unwrap().to_string(),
                });
            }
        }

        result.push(g);
    }

    result
}
