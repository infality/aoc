pub fn task1(input: &str) {
    let mut fish = Vec::<usize>::new();

    for num in input.split(",") {
        fish.push(num.parse().unwrap());
    }

    for _ in 0..80 {
        for i in (0..fish.len()).rev() {
            if fish[i] == 0 {
                fish[i] = 6;
                fish.push(8);
            } else {
                fish[i] -= 1;
            }
        }
    }

    println!("Result: {}", fish.len());
}

pub fn task2(input: &str) {
    let mut fish = [0usize; 9];

    for num in input.split(",") {
        fish[num.parse::<usize>().unwrap()] += 1;
    }

    for _ in 0..256 {
        let mut last = fish[8];
        for i in (0..=8).rev() {
            if i == 0 {
                fish[8] = last;
                fish[6] += last;
            } else {
                let temp = fish[i - 1];
                fish[i- 1] = last;
                last = temp;
            }
        }
    }

    println!("Result: {}", fish.iter().sum::<usize>());
}
