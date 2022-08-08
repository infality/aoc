pub fn task1(input: &str) {
    let mut result = usize::MAX;

    let mut wires = Vec::new();
    for line in input.lines() {
        let mut positions = Vec::new();
        let mut x = 0;
        let mut y = 0;
        positions.push((x, y));
        for instruction in line.split(',') {
            let amount: isize = instruction
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();
            match instruction.chars().nth(0).unwrap() {
                'U' => y += amount,
                'R' => x += amount,
                'D' => y -= amount,
                'L' => x -= amount,
                _ => panic!(),
            }
            positions.push((x, y));
        }
        wires.push(positions);
    }

    for i0 in 0..(wires[0].len() - 1) {
        let w0 = wires[0][i0];
        let w0_next = wires[0][i0 + 1];

        for i1 in 0..(wires[1].len() - 1) {
            let w1 = wires[1][i1];
            let w1_next = wires[1][i1 + 1];
            if w0.0 == w0_next.0 {
                if w1.0 == w1_next.0
                    || ((w1.0 < w0.0) == (w1_next.0 < w0.0))
                    || w1.1 < w0.1.min(w0_next.1)
                    || w1_next.1 > w0.1.max(w0_next.1)
                {
                    continue;
                }
                result = result.min(distance(w0.0, w1.1));
            } else {
                if w1.1 == w1_next.1
                    || ((w1.1 < w0.1) == (w1_next.1 < w0.1))
                    || w1.0 < w0.0.min(w0_next.0)
                    || w1_next.0 > w0.0.max(w0_next.0)
                {
                    continue;
                }
                result = result.min(distance(w0.1, w1.0));
            }
        }
    }

    println!("Result: {}", result);
}

fn distance(x: isize, y: isize) -> usize {
    (x.abs() + y.abs()) as usize
}

pub fn task2(input: &str) {
    let mut result = usize::MAX;

    let mut wires = Vec::new();
    for line in input.lines() {
        let mut positions = Vec::new();
        let mut x = 0;
        let mut y = 0;
        positions.push((x, y));
        for instruction in line.split(',') {
            let amount: isize = instruction
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .unwrap();
            match instruction.chars().nth(0).unwrap() {
                'U' => y += amount,
                'R' => x += amount,
                'D' => y -= amount,
                'L' => x -= amount,
                _ => panic!(),
            }
            positions.push((x, y));
        }
        wires.push(positions);
    }

    let mut w0_dist = 0;
    for i0 in 0..(wires[0].len() - 1) {
        let w0 = wires[0][i0];
        let w0_next = wires[0][i0 + 1];

        if w0.0 == w0_next.0 {
            w0_dist += (w0_next.1 - w0.1).abs();
        } else {
            w0_dist += (w0_next.0 - w0.0).abs();
        }

        let mut w1_dist = 0;
        for i1 in 0..(wires[1].len() - 1) {
            let w1 = wires[1][i1];
            let w1_next = wires[1][i1 + 1];

            if w1.0 == w1_next.0 {
                w1_dist += (w1_next.1 - w1.1).abs();
            } else {
                w1_dist += (w1_next.0 - w1.0).abs();
            }

            if w0.0 == w0_next.0 {
                if w1.0 == w1_next.0
                    || ((w1.0 < w0.0) == (w1_next.0 < w0.0))
                    || w1.1 < w0.1.min(w0_next.1)
                    || w1_next.1 > w0.1.max(w0_next.1)
                {
                    continue;
                }
                let dist0 = w0_dist - (w0_next.1 - w1.1).abs();
                let dist1 = w1_dist - (w1_next.0 - w0.0).abs();
                result = result.min((dist0 + dist1) as usize);
            } else {
                if w1.1 == w1_next.1
                    || ((w1.1 < w0.1) == (w1_next.1 < w0.1))
                    || w1.0 < w0.0.min(w0_next.0)
                    || w1_next.0 > w0.0.max(w0_next.0)
                {
                    continue;
                }
                let dist0 = w0_dist - (w0_next.0 - w1.0).abs();
                let dist1 = w1_dist - (w1_next.1 - w0.1).abs();
                result = result.min((dist0 + dist1) as usize);
            }
        }
    }

    println!("Result: {}", result);
}
