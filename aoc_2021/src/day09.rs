use std::collections::{HashSet, VecDeque};

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for line in input.split('\n') {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push(entry.to_string().parse::<usize>().unwrap());
        }
        data.push(row);
    }

    for (i, row) in data.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if i > 0 && data[i - 1][j] <= *entry {
                continue;
            }
            if i < data.len() - 1 && data[i + 1][j] <= *entry {
                continue;
            }
            if j > 0 && data[i][j - 1] <= *entry {
                continue;
            }
            if j < data[0].len() - 1 && data[i][j + 1] <= *entry {
                continue;
            }
            result += entry + 1;
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 1usize;

    let mut data = Vec::new();
    for line in input.split('\n') {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push(entry.to_string().parse::<usize>().unwrap());
        }
        data.push(row);
    }

    let mut basins = Vec::new();
    for (i, row) in data.iter().enumerate() {
        for (j, entry) in row.iter().enumerate() {
            if *entry == 9
                || basins
                    .iter()
                    .any(|x: &Vec<(usize, usize)>| x.iter().any(|y| y.0 == i && y.1 == j))
            {
                continue;
            }

            let mut basin = Vec::new();
            let mut to_explore = Vec::new();
            to_explore.push((i, j));
            while let Some((x, y)) = to_explore.pop() {
                if data[x][y] == 9 || basin.contains(&(x, y)) {
                    continue;
                }

                basin.push((x, y));
                if x > 0 && !to_explore.contains(&(x - 1, y)) {
                    to_explore.push((x - 1, y));
                }
                if x < data.len() - 1 && !to_explore.contains(&(x + 1, y)) {
                    to_explore.push((x + 1, y));
                }
                if y > 0 && !to_explore.contains(&(x, y - 1)) {
                    to_explore.push((x, y - 1));
                }
                if y < data[0].len() - 1 && !to_explore.contains(&(x, y + 1)) {
                    to_explore.push((x, y + 1));
                }
            }
            basins.push(basin);
        }
    }

    basins.sort_by(|a, b| b.len().cmp(&a.len()));
    for basin in basins.iter().take(3) {
        result *= basin.len();
    }

    println!("Result: {}", result);
}
