#[derive(Debug)]
struct Tile {
    id: usize,
    sides: [u16; 4],  // clockwise
    rotations: usize, // 0,1,2,3
    flipped: bool,
}

impl Tile {
    // direction: 0=top,1=right..
    fn align(&mut self, direction: usize, side: u16) -> bool {
        for flipped in [false, true] {
            for rotations in 0..4 {
                let mut own_side = self.sides[(direction + rotations) % 4];
                if flipped {
                    own_side = 1024 - own_side;
                }
                if side == own_side {
                    self.rotations = rotations;
                    self.flipped = flipped;
                    return true;
                }
            }
        }
        false
    }
}

fn map(c: char) -> char {
    match c {
        '.' => '0',
        '#' => '1',
        _ => panic!(),
    }
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut tiles = Vec::new();
    for tile_info in input.split("\n\n") {
        let mut id = 0;
        let mut sides = [0u16; 4];
        for (i, line) in tile_info.lines().enumerate() {
            if i == 0 {
                id = line
                    .chars()
                    .skip(5)
                    .take(4)
                    .collect::<String>()
                    .parse()
                    .unwrap();
                continue;
            }
            if i == 1 {
                sides[0] =
                    u16::from_str_radix(&line.chars().map(map).collect::<String>(), 2).unwrap();
            }
            if i == 10 {
                sides[2] =
                    u16::from_str_radix(&line.chars().map(map).collect::<String>(), 2).unwrap();
            }
            if line.chars().nth(0).unwrap() == '#' {
                sides[3] += 2u16.pow(i as u32 - 1);
            }
            if line.chars().nth(9).unwrap() == '#' {
                sides[1] += 2u16.pow(10 - i as u32);
            }
        }
        tiles.push(Tile {
            id,
            sides,
            rotations: 0,
            flipped: false,
        });
    }

    let mut corners = Vec::new();
    for tile1 in tiles.iter() {
        let mut matches = 0;
        for direction1 in 0..4 {
            let side = tile1.sides[direction1];
            for tile2 in tiles.iter() {
                for flipped in [false, true] {
                    for direction2 in 0..4 {
                        let mut own_side = tile2.sides[direction2];
                        if flipped != (direction1 == 1 || direction1 == 3) {
                            own_side = 1024 - own_side;
                        }
                        if own_side == side {
                            matches += 1;
                        }
                    }
                }
            }
        }
        if matches < 2 {
            panic!();
        }
        if matches > 4 {
            panic!();
        }
        if matches == 2 {
            corners.push(tile1.clone());
        }
    }
    println!("{:?}", corners);

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
