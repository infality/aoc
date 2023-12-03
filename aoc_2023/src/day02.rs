pub fn task1(input: &str) {
    let mut result = 0usize;

    for (i, line) in input.lines().enumerate() {
        let rounds = line.split_once(": ").unwrap().1.split(";");
        let mut possible = true;
        for round in rounds {
            for round_info in round.split(",") {
                let (amount, color) = round_info.trim().split_once(" ").unwrap();
                let amount: usize = amount.parse().unwrap();
                match color {
                    "red" => possible &= amount <= 12,
                    "green" => possible &= amount <= 13,
                    "blue" => possible &= amount <= 14,
                    _ => panic!(),
                };
            }
        }
        if possible {
            result += i + 1;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let rounds = line.split_once(": ").unwrap().1.split(";");
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for round in rounds {
            for round_info in round.split(",") {
                let (amount, color) = round_info.trim().split_once(" ").unwrap();
                let amount: usize = amount.parse().unwrap();
                match color {
                    "red" => red = red.max(amount),
                    "green" => green = green.max(amount),
                    "blue" => blue = blue.max(amount),
                    _ => panic!(),
                };
            }
        }
        result += red * green * blue;
    }

    println!("Result: {}", result);
}
