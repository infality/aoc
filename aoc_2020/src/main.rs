use std::env;
mod day16;
mod day18;
mod day19;
mod day20;
mod day23;
mod day24;
mod day25;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Pass number of day");
        return;
    }

    let input = include_str!("input.txt").trim_end();

    match args[1].as_str() {
        "16" => {
            day16::task1(input);
            day16::task2(input);
        }
        "18" => {
            day18::task1(input);
            day18::task2(input);
        }
        "19" => {
            day19::task1(input);
            day19::task2(input);
        }
        "20" => {
            day20::task1(input);
            day20::task2(input);
        }
        "23" => {
            day23::task1(input);
            day23::task2(input);
        }
        "24" => {
            day24::task1(input);
            day24::task2(input);
        }
        "25" => {
            day25::task1(input);
            day25::task2(input);
        }
        _ => println!("Invalid day"),
    }
}
