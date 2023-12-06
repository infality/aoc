pub fn task1(input: &str) {
    let mut result = 0usize;

    for line in input.lines() {
        let (wins, nums) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let wins = wins
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut matches = 0;
        for num in nums.split(" ") {
            if num.is_empty() {
                continue;
            }
            if wins.contains(&num.parse::<usize>().unwrap()) {
                matches += 1;
            }
        }
        if matches > 0 {
            result += 2_i32.pow(matches - 1) as usize;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut amount = vec![1usize; input.lines().count()];
    for (i, line) in input.lines().enumerate() {
        let (wins, nums) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();

        let wins = wins
            .split(" ")
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut matches = 0;
        for num in nums.split(" ") {
            if num.is_empty() {
                continue;
            }
            if wins.contains(&num.parse::<usize>().unwrap()) {
                matches += 1;
            }
        }
        result += amount[i];
        if matches > 0 {
            for j in (i + 1)..amount.len().min(i + matches + 1) {
                amount[j] += amount[i];
            }
        }
    }

    println!("Result: {}", result);
}
