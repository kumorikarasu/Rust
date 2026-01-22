use std::{fs::File, io::Read};

use super::r#trait::Day;

pub struct Day1 {}

impl Day for Day1 {
    fn main(&self, file_name: &str) {
        let mut file = File::open(file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut sum = 0;
        for line in contents.lines() {
            let mut first = ' ';
            let mut last = ' ';
            for c in line.chars() {
                if c.is_digit(10) {
                    last = c;
                    if first == ' ' {
                        first = c;
                    }
                }
            }

            let digit = first.to_digit(10).unwrap_or(0) * 10 + last.to_digit(10).unwrap_or(0);
            sum += digit;
        }

        println!("Total sum of {}: {}", file_name, sum);

        let mut sum = 0;
        // Get matches including words

        for line in contents.lines() {
            let (first, last) = get_first_last_instance_of(line);

            let digit = first * 10 + last;
            sum += digit;
        }

        println!("Total sum of {} including words: {}", file_name, sum);
    }
}

pub fn get_first_last_instance_of(string: &str) -> (i32, i32) {
    // digit can be a single char or the digit spelled out
    // e.g. "1" or "one"
    let matches = vec!("0", "zero", "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven", "8", "eight", "9", "nine");

    let mut first = -1;
    let mut last = -1;
    // slice the string and walk through the matches
    for i in 0..string.len() {
        let s = &string[i..];
        for (j, m) in matches.iter().enumerate() {
            if s.starts_with(m) {
                if first == -1 {
                    first = (j as i32) / 2;
                }
                last = (j as i32) / 2;
            }
        }
    }

    (first, last)
}
