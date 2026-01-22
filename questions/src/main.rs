mod days;

use std::fs;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    day: Option<u8>,
}


fn main() {
    let args = Args::parse();

    let files = fs::read_dir("input").unwrap();
    match args.day {
        Some(x) => {
            // Run every file that starts with dayX in input folder
            let day = days::get_day(x);
            for file in files {
                let file = file.unwrap();
                let path = file.path();
                let path_str = path.to_str().unwrap();
                let m = format!("input/day{}", x.to_string());
                if path_str.starts_with(&m) {
                    day.main(path_str);
                }
            }
        }
        _ => println!("No day specified"),
    }
}
