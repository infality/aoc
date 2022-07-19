#[derive(Clone, PartialEq, Debug)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    pub fn diff(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn manhattan(&self, other: &Position) -> usize {
        (isize::abs(self.x - other.x) + isize::abs(self.y - other.y) + isize::abs(self.z - other.z))
            as usize
    }

    pub fn add(&self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn sub(&self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn is_equal_rotated(&self, other: &Position) -> bool {
        let mut other_values = vec![other.x.abs(), other.y.abs(), other.z.abs()];
        if let Some(pos) = other_values.iter().position(|x| *x == self.x.abs()) {
            other_values.remove(pos);
        } else {
            return false;
        }
        if let Some(pos) = other_values.iter().position(|y| *y == self.y.abs()) {
            other_values.remove(pos);
        } else {
            return false;
        }
        return *other_values.first().unwrap() == self.z.abs();
    }

    pub fn get_axis(&self, i: isize) -> isize {
        match i {
            1 => self.x,
            -1 => -self.x,
            2 => self.y,
            -2 => -self.y,
            3 => self.z,
            -3 => -self.z,
            _ => panic!(),
        }
    }

    pub fn rotated(&self, params: (isize, isize, isize)) -> Position {
        Position {
            x: self.get_axis(params.0),
            y: self.get_axis(params.1),
            z: self.get_axis(params.2),
        }
    }

    pub fn get_rotation_parameters(&self, other: &Position) -> Vec<(isize, isize, isize)> {
        let mut rotations = Vec::new();
        for x in 1..=3 {
            for x_axis in [x, -x] {
                for y in 1..=3 {
                    if x == y {
                        continue;
                    }
                    for y_axis in [y, -y] {
                        for z in 1..=3 {
                            if x == z || y == z {
                                continue;
                            }
                            for z_axis in [z, -z] {
                                if self.x == other.get_axis(x_axis)
                                    && self.y == other.get_axis(y_axis)
                                    && self.z == other.get_axis(z_axis)
                                {
                                    rotations.push((x_axis, y_axis, z_axis));
                                }
                            }
                        }
                    }
                }
            }
        }
        rotations
    }
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut scanner_beacons = Vec::new();
    for scanner_lines in input.split("\n\n") {
        let mut beacons = Vec::new();
        for line in scanner_lines.lines().skip(1) {
            let coordinates = line
                .splitn(3, ',')
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>();
            beacons.push(Position {
                x: coordinates[0],
                y: coordinates[1],
                z: coordinates[2],
            });
        }
        scanner_beacons.push(beacons);
    }

    let mut scanner_positions = vec![None; scanner_beacons.len()];
    scanner_positions[0] = Some(Position { x: 0, y: 0, z: 0 });
    while scanner_positions.iter().any(|x| x.is_none()) {
        let known_scanner_indices = (0..scanner_positions.len())
            .filter(|x| scanner_positions[*x].is_some())
            .collect::<Vec<usize>>();
        let unknown_scanner_indices = (0..scanner_positions.len())
            .filter(|x| scanner_positions[*x].is_none())
            .collect::<Vec<usize>>();
        let mut found_match = false;
        for i in known_scanner_indices.iter() {
            for j in unknown_scanner_indices.iter() {
                println!("Testing {},{}", i, j);

                // Match diffs
                let match_result = find_match(*i, *j, &scanner_beacons);
                if let Some(match_result) = match_result {
                    let j_rotation = match_result.2;

                    let beacon1 = &scanner_beacons[*i][match_result.0];
                    let beacon2 = &scanner_beacons[*j][match_result.1];
                    scanner_positions[*j] = Some(beacon1.sub(&beacon2.rotated(j_rotation)));

                    // Update beacons
                    for beacon in scanner_beacons[*j].iter_mut() {
                        *beacon = beacon
                            .rotated(j_rotation)
                            .add(scanner_positions[*j].as_ref().unwrap());
                    }

                    println!(
                        "Found {},{} {:?} {:?}",
                        i,
                        j,
                        scanner_positions[*j].as_ref().unwrap(),
                        j_rotation
                    );
                    found_match = true;
                    break;
                }
            }
            if found_match {
                break;
            }
        }
        if !found_match {
            panic!("No further match found");
        }
    }

    println!("{:?}", scanner_positions);

    let mut beacons = Vec::new();
    for i in 0..scanner_beacons.len() {
        for beacon in scanner_beacons[i].iter() {
            if !beacons.contains(beacon) {
                beacons.push(beacon.clone());
            }
        }
    }
    result = beacons.len();

    println!("Result: {}", result);

    result = 0;
    for i in scanner_positions.iter() {
        for j in scanner_positions.iter() {
            if i == j {
                continue;
            }
            result = result.max(i.as_ref().unwrap().manhattan(j.as_ref().unwrap()));
        }
    }

    println!("Result: {}", result);
}

fn find_match(
    i: usize,
    j: usize,
    scanner_beacons: &Vec<Vec<Position>>,
) -> Option<(usize, usize, (isize, isize, isize))> {
    let mut known_diffs = Vec::new();
    for i_beacon1 in scanner_beacons[i].iter() {
        let mut diffs = Vec::new();
        for i_beacon2 in scanner_beacons[i].iter() {
            diffs.push(i_beacon1.diff(i_beacon2));
        }
        known_diffs.push(diffs);
    }

    for (i1, i_diffs) in known_diffs.iter().enumerate() {
        for (i2, i_diff) in i_diffs.iter().enumerate() {
            if i1 == i2 {
                continue;
            }
            for (j1, j1_beacon) in scanner_beacons[j].iter().enumerate() {
                for (j2, j2_beacon) in scanner_beacons[j].iter().enumerate() {
                    if j1 == j2 {
                        continue;
                    }
                    let j_diff = j1_beacon.diff(j2_beacon);
                    if !i_diff.is_equal_rotated(&j_diff) {
                        continue;
                    }

                    let rotation_params = i_diff.get_rotation_parameters(&j_diff);

                    /* println!(
                        "Testing offset {:?}",
                        &i_diff.sub(&j_diff.rotated(rotation_params))
                    ); */

                    for rotation in rotation_params {
                        let mut matching_count = 2;
                        let mut matching_js = vec![j1, j2];
                        if j_diff.rotated(rotation) == *i_diff {
                            // Find matches with j1
                            for i3_diff in known_diffs[i1].iter() {
                                for (j3, j3_beacon) in scanner_beacons[j].iter().enumerate() {
                                    if j1 == j3 || matching_js.contains(&j3) {
                                        continue;
                                    }
                                    if *i3_diff == j1_beacon.diff(j3_beacon).rotated(rotation) {
                                        matching_js.push(j3);
                                        matching_count += 1;
                                    }
                                }
                            }

                            if matching_count >= 12 {
                                return Some((i1, j1, rotation));
                            }
                        } else {
                            // Find matches with j2
                            for i3_diff in known_diffs[i1].iter() {
                                for (j3, j3_beacon) in scanner_beacons[j].iter().enumerate() {
                                    if j2 == j3 || matching_js.contains(&j3) {
                                        continue;
                                    }
                                    if *i3_diff == j2_beacon.diff(j3_beacon).rotated(rotation) {
                                        matching_js.push(j3);
                                        matching_count += 1;
                                    }
                                }
                            }

                            if matching_count >= 12 {
                                return Some((i1, j2, rotation));
                            }
                        }
                    }
                }
            }
        }
    }
    None
}

pub fn task2(_input: &str) {}
