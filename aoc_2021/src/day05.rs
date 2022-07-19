pub fn task1(input: &str) {
    let mut area = [[0usize; 1000]; 1000];

    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let parts: Vec<&str> = line.split(" -> ").collect();
        let left: Vec<&str> = parts[0].split(",").collect();
        let right: Vec<&str> = parts[1].split(",").collect();

        let x1: usize = left[0].parse().unwrap();
        let y1: usize = left[1].parse().unwrap();
        let x2: usize = right[0].parse().unwrap();
        let y2: usize = right[1].parse().unwrap();

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                area[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                area[x][y1] += 1;
            }
        } else {
            println!("Skipping");
        }
    }

    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if area[x][y] > 1 {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut area = [[0usize; 1000]; 1000];

    for line in input.split("\n") {
        if line == "" {
            continue;
        }
        let parts: Vec<&str> = line.split(" -> ").collect();
        let left: Vec<&str> = parts[0].split(",").collect();
        let right: Vec<&str> = parts[1].split(",").collect();

        let x1: usize = left[0].parse().unwrap();
        let y1: usize = left[1].parse().unwrap();
        let x2: usize = right[0].parse().unwrap();
        let y2: usize = right[1].parse().unwrap();

        let xdiff = x1.max(x2) - x1.min(x2);
        let ydiff = y1.max(y2) - y1.min(y2);

        if x1 == x2 {
            for y in y1.min(y2)..=y1.max(y2) {
                area[x1][y] += 1;
            }
        } else if y1 == y2 {
            for x in x1.min(x2)..=x1.max(x2) {
                area[x][y1] += 1;
            }
        } else if xdiff == ydiff {
            for i in 0..=xdiff {
                if (x1 < x2) == (y1 < y2) {
                    area[x1.min(x2) + i][y1.min(y2) + i] += 1;
                } else {
                    area[x1.min(x2) + i][y1.max(y2) - i] += 1;
                }
            }
        } else {
            println!("Skipping");
        }
    }

    let mut result = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            if area[x][y] > 1 {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}
