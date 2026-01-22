use std::{fs::File, io::Read};

use super::r#trait::Day;

use std::rc::Rc;
use std::cell::RefCell;

// The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.
// The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

pub struct Day3 { }

#[derive(Debug)]
struct Game {
    width: i32,
    height: i32,
    items: Vec<Rc<RefCell<GameItem>>>,
}

#[derive(Debug)]
struct GameItem {
    id: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    number: Option<i32>,
    symbol: Option<char>,
    text: String, // Replace with char[] for optimization
}

impl PartialEq for GameItem {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Day for Day3 {
    fn main(&self, file_name: &str) {
        let game = parse_file(file_name);

        let mut products = Vec::new();
        let mut collisions = Vec::new();

        for item in &game.items {
            if item.borrow().symbol.is_some() {
                // Find collisions and store them locally
                let mut local_collisions = Vec::new();
                let x = item.borrow().x - 1;
                let y = item.borrow().y - 1;
                let width = item.borrow().width + 2;
                let height = item.borrow().height + 2;


                for item2 in &game.items {
                    if item2.borrow().number.is_some() {
                        let x2 = item2.borrow().x;
                        let y2 = item2.borrow().y;
                        let width2 = item2.borrow().width;
                        let height2 = item2.borrow().height;

                        // Do the rectangles intersect?
                        if x < x2 + width2 && x + width > x2 && y < y2 + height2 && y + height > y2 {
                            collisions.push(item2.clone());
                            local_collisions.push(item2.clone());
                        }
                    }
                }

                // Product the local collisions
                if local_collisions.len() < 2 {
                    continue;
                }

                let product = local_collisions.iter().fold(1, |a, b| {
                    let item = b.borrow();
                    if item.number.is_some() {
                        return a * item.number.unwrap();
                    }
                    a
                });
                products.push(product);
            }
        }

        // Collisions remove duplicate ids
        let mut dedub_collisions= Vec::new();
        for item in collisions {
            if !dedub_collisions.contains(&item) {
                dedub_collisions.push(item);
            }
        }

        let sum = dedub_collisions.iter().fold(0, |a, b| {
            let item = b.borrow();
            if item.number.is_some() {
                return a + item.number.unwrap();
            }
            a
        });
        println!("Sum of collisions for {} : {}", file_name, sum);


        // Sum products
        let sum = products.iter().fold(0, |a, b| {
            return a + b;
        });
        println!("Sum of gear ratio products for {} : {}", file_name, sum);
    }
}

fn parse_file(file_name: &str) -> Game {
    let mut g = Game {
        width: 0,
        height: 0,
        items: Vec::new(),
    };

    let mut file = File::open(file_name).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut y = 0;
    let mut id = 0;
    for line in contents.lines() {
        g.height += 1;
        g.width = line.len() as i32;

        // Look at the current char of the line, if it is a . then skip it
        // If it is a number, then calculate the width and add it to the list of items
        // If it is a symbol, then add it to the list of items

        let mut x = 0;
        let mut current: Option<Rc<RefCell<GameItem>>> = None;
        for c in line.chars() {
            id += 1;
            if c == '.' {
                current = None;
                x += 1;
                continue;
            }
            if c.is_digit(10) {
                if current.is_none() {
                    let item = Rc::new(RefCell::new(GameItem {
                        id,
                        x,
                        y,
                        width: 1,
                        height: 1,
                        number: Some(c.to_digit(10).unwrap() as i32),
                        symbol: None,
                        text: c.to_string(),
                    }));
                    current = Some(item.clone());
                    g.items.push(item);
                } else {
                    let item = current.clone().unwrap();
                    let mut item = item.borrow_mut();
                    item.width += 1;
                    item.text.push(c);
                    item.number = Some(item.text.parse::<i32>().unwrap());
                }
            }

            if c.is_ascii_punctuation() {
                let item = Rc::new(RefCell::new(GameItem {
                    id,
                    x,
                    y,
                    width: 1,
                    height: 1,
                    number: None,
                    symbol: Some(c),
                    text: c.to_string(),
                }));
                g.items.push(item);
                current = None;
            }
            x += 1;
        }
        y += 1;
    }

    g
}
