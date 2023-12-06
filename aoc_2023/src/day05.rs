pub fn task1(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n").unwrap();
    let mut values = parts
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    for map_info in parts.1.trim().split("\n\n") {
        for value in values.iter_mut() {
            for map in map_info.split_once("\n").unwrap().1.lines() {
                let map_parts = map
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
                let destination = map_parts[0];
                let source = map_parts[1];
                let length = map_parts[2];
                if source <= *value && *value < source + length {
                    let diff = *value - source;
                    *value = destination + diff;
                    break;
                }
            }
        }
    }

    result = *values.iter().min().unwrap();

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let parts = input.split_once("\n").unwrap();
    let values = parts
        .0
        .split_once(": ")
        .unwrap()
        .1
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();
    let mut ranges = values
        .chunks(2)
        .map(|x| (x[0], x[1]))
        .collect::<Vec<(usize, usize)>>();

    for map_info in parts.1.trim().split("\n\n") {
        let mut new_ranges = Vec::new();
        while let Some(range) = ranges.pop() {
            let mut found_map = false;
            for map in map_info.split_once("\n").unwrap().1.lines() {
                let map_parts = map
                    .split(" ")
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>();
                let destination = map_parts[0];
                let source = map_parts[1];
                let length = map_parts[2];

                // Outside
                if range.0 + range.1 <= source || source + length <= range.0 {
                    continue;
                }

                // Full map
                if source <= range.0 && range.0 + range.1 <= source + length {
                    let diff = range.0 - source;
                    new_ranges.push((destination + diff, range.1));
                    found_map = true;
                    break;
                }
                // Partial map at beginning
                if source <= range.0 && range.0 < source + length {
                    let diff = range.0 - source;
                    let part_length = source + length - range.0;
                    new_ranges.push((destination + diff, part_length));
                    ranges.push((range.0 + part_length, range.1 - part_length));
                    found_map = true;
                    break;
                }
                // Partial map at end
                if source < range.0 + range.1 && range.0 + range.1 <= source + length {
                    let part_length = range.0 + range.1 - source;
                    new_ranges.push((destination, part_length));
                    ranges.push((range.0, range.1 - part_length));
                    found_map = true;
                    break;
                }
                // map in middle
                if range.0 < source && source + length < range.0 + range.1 {
                    let left_length = source - range.0;
                    let right_length = range.0 + range.1 - (source + length);
                    new_ranges.push((destination, length));
                    ranges.push((range.0, left_length));
                    ranges.push((source + length, right_length));
                    found_map = true;
                    break;
                }
                unreachable!();
            }
            if !found_map {
                new_ranges.push(range);
            }
        }
        ranges = new_ranges;
    }

    result = ranges.iter().min_by_key(|x| x.0).unwrap().0;

    println!("Result: {}", result);
}
