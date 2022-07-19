use std::collections::HashMap;

pub fn task1(input: &str) {
    let mut result = 0usize;

    for line in input.split('\n') {
        let output = line.split(" | ").collect::<Vec<&str>>()[1];
        for entry in output.split(' ') {
            if entry.len() == 2 || entry.len() == 4 || entry.len() == 3 || entry.len() == 7 {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    for line in input.split('\n') {
        let left = line.split(" | ").collect::<Vec<&str>>()[0];

        let mut map = HashMap::new();

        let left_entries = left.split(' ').collect::<Vec<&str>>();
        let one = left_entries.iter().find(|x| x.len() == 2).unwrap(); // 1
        let sevene = left_entries.iter().find(|x| x.len() == 3).unwrap(); // 7
        let four = left_entries.iter().find(|x| x.len() == 4).unwrap(); // 4
        let entries_5 = left_entries
            .iter()
            .filter(|x| x.len() == 5)
            .cloned()
            .collect::<Vec<&str>>(); // 2,3,5
        let entries_6 = left_entries
            .iter()
            .filter(|x| x.len() == 6)
            .cloned()
            .collect::<Vec<&str>>(); // 0,6,9
        let eight = left_entries.iter().find(|x| x.len() == 7).unwrap(); // 8

        map.insert('a', sevene.chars().find(|x| !one.contains(*x)).unwrap());
        let cf = sevene
            .chars()
            .filter(|x| one.contains(*x))
            .collect::<Vec<char>>();
        let bd = four
            .chars()
            .filter(|x| !one.contains(*x))
            .collect::<Vec<char>>();

        {
            let abcdf = vec![map.get(&'a').unwrap().clone(), cf[0], cf[1], bd[0], bd[1]];
            let nine = entries_6
                .iter()
                .find(|x| {
                    for i in abcdf.iter() {
                        if !x.contains(&i.to_string()) {
                            return false;
                        }
                    }
                    true
                })
                .unwrap();
            map.insert('g', nine.chars().find(|x| !abcdf.contains(&x)).unwrap());
        }

        {
            let acfg = vec![
                map.get(&'a').unwrap().clone(),
                cf[0],
                cf[1],
                map.get(&'g').unwrap().clone(),
            ];
            let three = entries_5
                .iter()
                .find(|x| {
                    for i in acfg.iter() {
                        if !x.contains(&i.to_string()) {
                            return false;
                        }
                    }
                    true
                })
                .unwrap();
            map.insert('d', three.chars().find(|x| !acfg.contains(&x)).unwrap());
            map.insert(
                'b',
                bd.iter()
                    .find(|x| *x != map.get(&'d').unwrap())
                    .unwrap()
                    .clone(),
            );
        }

        {
            let abdg = vec![
                map.get(&'a').unwrap().clone(),
                map.get(&'b').unwrap().clone(),
                map.get(&'d').unwrap().clone(),
                map.get(&'g').unwrap().clone(),
            ];
            let five = entries_5
                .iter()
                .find(|x| {
                    for i in abdg.iter() {
                        if !x.contains(&i.to_string()) {
                            return false;
                        }
                    }
                    true
                })
                .unwrap();
            map.insert('f', five.chars().find(|x| !abdg.contains(&x)).unwrap());
            map.insert(
                'c',
                cf.iter()
                    .find(|x| *x != map.get(&'f').unwrap())
                    .unwrap()
                    .clone(),
            );
        }

        {
            map.insert(
                'e',
                eight
                    .chars()
                    .find(|x| !map.values().any(|i| i == x))
                    .unwrap(),
            );
        }

        let right = line.split(" | ").collect::<Vec<&str>>()[1];
        let mut value = 0;
        for (i, entry) in right.split(' ').rev().enumerate() {
            let digit;
            if entry.len() == 2 {
                digit = 1;
            } else if entry.len() == 3 {
                digit = 7;
            } else if entry.len() == 4 {
                digit = 4;
            } else if entry.len() == 5 {
                if entry.chars().any(|x| x == *map.get(&'b').unwrap()) {
                    digit = 5;
                } else if entry.chars().any(|x| x == *map.get(&'e').unwrap()) {
                    digit = 2;
                } else {
                    digit = 3;
                }
            } else if entry.len() == 6 {
                if entry.chars().any(|x| x == *map.get(&'d').unwrap()) {
                    if entry.chars().any(|x| x == *map.get(&'c').unwrap()) {
                        digit = 9;
                    } else {
                        digit = 6;
                    }
                } else {
                    digit = 0;
                }
            } else {
                digit = 8;
            }
            value += digit * 10usize.pow(i as u32);
            println!("{}", value);
        }
        result += value;
    }

    println!("Result: {}", result);
}
