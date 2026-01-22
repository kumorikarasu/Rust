use std::{fs::File, io::Read};

use super::r#trait::Day;


// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

#[derive(Debug)]
pub struct Game {
    card: usize,
    
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
    matches: Vec<u32>,
}

pub struct Day4 { }


impl Day for Day4 {
    fn main(&self, file_name: &str) {
        let mut games = parse_file(file_name);
        let mut p1sum = 0;
        for game in &mut games {
            for number in &game.numbers {
                if game.winning_numbers.contains(&number) {
                    game.matches.push(*number);
                }
            }

            if game.matches.len() != 0 {
                p1sum += usize::pow(2, (game.matches.len()-1) as u32);
            }
            //game.matches.replace(matches);
        }
        println!("Sum of winning tickets for {}: {}", file_name, p1sum);

        // Part 2
        let mut p2sum = 0;
        for game in &games {
            p2sum += game_calc(game, &games);
        }

        println!("Sum of winning tickets for {}: {}", file_name, p2sum);
    }
}

fn game_calc(game: &Game, games: &Vec<Game>) -> usize {
    let mut sum = 1;
    if game.matches.len() != 0 {
        // Find the next N cards that score
        let mut next_cards = Vec::new();
        for (i, _) in game.matches.iter().enumerate() {
            next_cards.push(games.iter().find(|x| x.card == game.card + i + 1).unwrap());
        }
        for next_card in next_cards {
            sum += game_calc(next_card, games);
        }
    }
    sum
}

fn parse_file(file_name: &str) -> Vec<Game>{
    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut games = Vec::new();

    for line in contents.lines() {
        // Split on : and |
        // Parse into Game struct
        let mut card = line.split(" ");
        card.next();

        let mut card = line.split(":");
        let mut card_id = card.next().unwrap().split_whitespace();
        card_id.next();

        let id = card_id.next().unwrap().parse::<usize>().unwrap();

        let mut numbers = card.next().unwrap().split("|");

        let w = numbers.next().unwrap().trim().split(" ").map(|x| x.parse::<u32>().ok()).collect::<Vec<Option<u32>>>();
        let winning_numbers = w.iter().filter_map(|x| *x).collect::<Vec<u32>>();


        let n = numbers.next().unwrap().trim().split(" ").map(|x| x.parse::<u32>().ok()).collect::<Vec<Option<u32>>>();

        let numbers = n.iter().filter_map(|x| *x).collect::<Vec<u32>>();


        let game = Game {
            card: id,
            winning_numbers,
            numbers,
            matches: Vec::new(),
        };

        games.push(game);
    }

    games
}
