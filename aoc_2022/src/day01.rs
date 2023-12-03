pub fn task1(input: &str) {
    let mut result = 0usize;

    for elf in input.split("\n\n") {
        let mut amount = 0;
        for line in elf.lines() {
            amount += line.parse::<usize>().unwrap();
        }
        result = result.max(amount);
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut elves = Vec::new();
    for elf in input.split("\n\n") {
        let mut amount = 0;
        for line in elf.lines() {
            amount += line.parse::<usize>().unwrap();
        }
        elves.push(amount);
    }
    elves.sort_unstable();
    result = elves.iter().rev().take(3).sum();

    println!("Result: {}", result);
}
