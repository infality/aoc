fn parse_line(line: &str) -> Vec<usize> {
    line.split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn task1(input: &str) {
    let mut result = 1usize;

    let times = parse_line(input.lines().nth(0).unwrap());
    let distances = parse_line(input.lines().nth(1).unwrap());

    for (time, distance) in times.iter().zip(distances) {
        let mut ways = 0;
        for holding in 1..*time {
            if (time - holding) * holding > distance {
                ways += 1;
            }
        }
        result *= ways;
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let time: usize = input
        .lines()
        .nth(0)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();
    let distance: usize = input
        .lines()
        .nth(1)
        .unwrap()
        .split_once(":")
        .unwrap()
        .1
        .replace(" ", "")
        .parse()
        .unwrap();

    for holding in 1..time {
        if (time - holding) * holding > distance {
            result += 1;
        }
    }

    println!("Result: {}", result);
}
