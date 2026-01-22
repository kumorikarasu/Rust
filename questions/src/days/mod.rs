pub mod r#trait;
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;


pub fn get_day(day: u8) -> Box<dyn r#trait::Day> {
    match day {
        1 => Box::new(day1::Day1 {}),
        2 => Box::new(day2::Day2 {}),
        3 => Box::new(day3::Day3 {}),
        4 => Box::new(day4::Day4 {}),
        _ => panic!("Day {} not implemented", day),
    }
}

