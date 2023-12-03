use std::env;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
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
        "1" => {
            day01::task1(input);
            day01::task2(input);
        }
        "2" => {
            day02::task1(input);
            day02::task2(input);
        }
        "3" => {
            day03::task1(input);
            day03::task2(input);
        }
        "4" => {
            day04::task1(input);
            day04::task2(input);
        }
        "5" => {
            day05::task1(input);
            day05::task2(input);
        }
        "6" => {
            day06::task1(input);
            day06::task2(input);
        }
        "7" => {
            day07::task1(input);
            day07::task2(input);
        }
        "8" => {
            day08::task1(input);
            day08::task2(input);
        }
        "9" => {
            day09::task1(input);
            day09::task2(input);
        }
        "10" => {
            day10::task1(input);
            day10::task2(input);
        }
        "11" => {
            day11::task1(input);
            day11::task2(input);
        }
        "12" => {
            day12::task1(input);
            day12::task2(input);
        }
        "13" => {
            day13::task1(input);
            day13::task2(input);
        }
        "14" => {
            day14::task1(input);
            day14::task2(input);
        }
        "15" => {
            day15::task1(input);
            day15::task2(input);
        }
        "16" => {
            day16::task1(input);
            day16::task2(input);
        }
        "17" => {
            day17::task1(input);
            day17::task2(input);
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
        "21" => {
            day21::task1(input);
            day21::task2(input);
        }
        "22" => {
            day22::task1(input);
            day22::task2(input);
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
