use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Tile {
    id: usize,
    sides: [u16; 4],  // clockwise
    rotations: usize, // 0,1,2,3
    flipped: bool,
    data: [[char; 10]; 10],
}

impl Tile {
    fn read(input: &str) -> Vec<Tile> {
        let mut tiles = Vec::new();
        for tile_info in input.split("\n\n") {
            let mut id = 0;
            let mut sides = [0u16; 4];
            let mut data = [['-'; 10]; 10];
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
                for (ci, c) in line.chars().enumerate() {
                    data[i - 1][ci] = c;
                }
                if i == 1 {
                    sides[0] =
                        u16::from_str_radix(&line.chars().map(map).collect::<String>(), 2).unwrap();
                }
                if i == 10 {
                    sides[2] = reverse_bits(
                        u16::from_str_radix(&line.chars().map(map).collect::<String>(), 2).unwrap(),
                    );
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
                data,
            });
        }
        tiles
    }

    // direction: 0=top,1=right..
    fn align(&mut self, direction: usize, side: u16) -> bool {
        for flipped in [false, true] {
            for test_direction in 0..4 {
                let test_side = if flipped {
                    let mut i = test_direction;
                    if i % 2 == 1 {
                        i = (i + 2) % 4;
                    }
                    reverse_bits(self.sides[i])
                } else {
                    self.sides[test_direction]
                };
                if test_side == side {
                    self.flipped = flipped;
                    self.rotations = (4 + direction - test_direction) % 4;
                    return true;
                }
            }
        }
        false
    }

    fn get_side(&self, direction: usize) -> u16 {
        let mut i = (4 + direction - self.rotations) % 4;
        if self.flipped {
            if i % 2 == 1 {
                i = (i + 2) % 4;
            }
            reverse_bits(self.sides[i])
        } else {
            self.sides[i]
        }
    }
}

fn map(c: char) -> char {
    match c {
        '.' => '0',
        '#' => '1',
        _ => panic!(),
    }
}

fn reverse_bits(mut num: u16) -> u16 {
    let mut result = 0;

    for _ in 0..10 {
        result <<= 1;

        if num > 0 {
            if (num & 1) == 1 {
                result ^= 1;
            }
            num >>= 1;
        }
    }
    result
}

pub fn task1(input: &str) {
    let mut result = 0usize;
    let tiles = Tile::read(input);

    let mut corners = Vec::new();
    for tile1 in tiles.iter() {
        let mut matches = 0;
        for direction1 in 0..4 {
            let side = reverse_bits(tile1.sides[direction1]);
            for tile2 in tiles.iter() {
                if tile1.id == tile2.id {
                    continue;
                }
                for flipped in [false, true] {
                    for direction2 in 0..4 {
                        let test_side = if flipped {
                            let mut i = direction2;
                            if i % 2 == 1 {
                                i = (i + 2) % 4;
                            }
                            reverse_bits(tile2.sides[i])
                        } else {
                            tile2.sides[direction2]
                        };
                        if test_side == side {
                            /* println!(
                                "--{} {} {} {} {}",
                                tile1.id, direction1, tile2.id, flipped, direction2
                            ); */
                            matches += 1;
                        }
                    }
                }
            }
        }
        if matches < 2 {
            panic!("{}", matches);
        }
        if matches > 4 {
            panic!("{}", matches);
        }
        if matches == 2 {
            corners.push(tile1.clone());
        }
    }
    result = corners.iter().map(|x| x.id).product();

    println!("Result: {}", result);
}

const WIDTH: usize = 12;
pub fn task2(input: &str) {
    let mut result = 0usize;
    let mut tiles = Tile::read(input);

    let mut matches = HashMap::new();
    let mut first_corner_direction = None;
    for tile1 in tiles.iter() {
        let mut match_count = 0;
        let mut min_direction = None;
        for direction1 in 0..4 {
            let side = reverse_bits(tile1.sides[direction1]);
            for tile2 in tiles.iter() {
                if tile1.id == tile2.id {
                    continue;
                }
                for flipped in [false, true] {
                    for direction2 in 0..4 {
                        let test_side = if flipped {
                            let mut i = direction2;
                            if i % 2 == 1 {
                                i = (i + 2) % 4;
                            }
                            reverse_bits(tile2.sides[i])
                        } else {
                            tile2.sides[direction2]
                        };
                        if test_side == side {
                            /* println!(
                                "--{} {} {} {} {}",
                                tile1.id, direction1, tile2.id, flipped, direction2
                            ); */
                            match_count += 1;
                            if min_direction.is_none() || direction1 + 1 == min_direction.unwrap() {
                                min_direction = Some(direction1);
                            }
                        }
                    }
                }
            }
        }
        matches.insert(tile1.id, match_count);
        if first_corner_direction.is_none() && match_count == 2 {
            first_corner_direction = min_direction;
        }
    }

    let mut area = [[None; WIDTH]; WIDTH];
    {
        let mut tile = tiles.remove(tiles.iter().position(|x| matches[&x.id] == 2).unwrap());
        tile.rotations = (5 - first_corner_direction.unwrap()) % 4;
        area[0][0] = Some(tile);
    }

    // Fill edges
    for edge in 0..4 {
        for i in 1..WIDTH {
            if edge == 3 && i == WIDTH - 1 {
                continue;
            }
            let direction = (edge + 3) % 4;
            let neighbor = match edge {
                0 => area[0][i - 1].unwrap(),
                1 => area[i - 1][WIDTH - 1].unwrap(),
                2 => area[WIDTH - 1][WIDTH - i].unwrap(),
                3 => area[WIDTH - i][0].unwrap(),
                _ => panic!(),
            };

            let mut found = false;
            for j in 0..tiles.len() {
                if matches[&tiles[j].id] > 3 {
                    continue;
                }
                if tiles[j].align(
                    direction,
                    reverse_bits(neighbor.get_side((direction + 2) % 4)),
                ) {
                    let tile = Some(tiles.remove(j));
                    match edge {
                        0 => area[0][i] = tile,
                        1 => area[i][WIDTH - 1] = tile,
                        2 => area[WIDTH - 1][WIDTH - 1 - i] = tile,
                        3 => area[WIDTH - 1 - i][0] = tile,
                        _ => panic!(),
                    };
                    found = true;
                    break;
                }
            }
            if !found {
                print_area(&area);
                panic!("{} {}", edge, i,);
            }
        }
    }
    //print_area(&area);

    for row in 1..WIDTH - 1 {
        for col in 1..WIDTH - 1 {
            let mut found = false;
            for j in 0..tiles.len() {
                if tiles[j].align(3, reverse_bits(area[row][col - 1].unwrap().get_side(1))) {
                    let tile = Some(tiles.remove(j));
                    area[row][col] = tile;
                    found = true;
                    break;
                }
            }
            if !found {
                print_area(&area);
                panic!("{} {}", row, col);
            }
        }
    }
    //print_area(&area);

    get_image(&area, 0, false);

    println!("Result: {}", result);
}

fn print_area(area: &[[Option<Tile>; WIDTH]; WIDTH]) {
    for row in area.iter() {
        for cell in row.iter() {
            print!(" ");
            if let Some(cell) = cell {
                print!(
                    " {} {} {} ",
                    cell.id,
                    cell.rotations,
                    if cell.flipped { 1 } else { 0 }
                );
            } else {
                print!("          ");
            }
        }
        print!("\n");
        for data_row in 0..10 {
            for cell in row.iter() {
                print!(" ");
                if let Some(cell) = cell {
                    let mut horizontal = true;
                    let mut i = data_row;
                    let mut reverse = false;
                    match cell.rotations {
                        0 => (),
                        1 => {
                            horizontal = false;
                            reverse = true;
                        }
                        2 => {
                            i = 9 - i;
                            reverse = true;
                        }
                        3 => {
                            horizontal = false;
                            i = 9 - i;
                        }
                        _ => panic!(),
                    }
                    if cell.flipped {
                        if horizontal {
                            reverse = !reverse;
                        } else {
                            i = 9 - i;
                        }
                    }

                    if horizontal {
                        if reverse {
                            print!("{}", cell.data[i].iter().rev().collect::<String>());
                        } else {
                            print!("{}", cell.data[i].iter().collect::<String>());
                        }
                    } else {
                        if reverse {
                            print!(
                                "{}",
                                cell.data.iter().map(|x| x[i]).rev().collect::<String>()
                            );
                        } else {
                            print!("{}", cell.data.iter().map(|x| x[i]).collect::<String>());
                        }
                    }
                } else {
                    print!("          ");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn get_image(
    area: &[[Option<Tile>; WIDTH]; WIDTH],
    rotation: usize,
    flipped: bool,
) -> Vec<Vec<bool>> {
    let mut result = Vec::new();
    for row in area.iter() {
        for data_row in 0..10 {
            let mut result_row = Vec::new();
            for cell in row.iter() {
                let mut horizontal = true;
                let mut i = data_row;
                let mut reverse = false;
                match cell.unwrap().rotations {
                    0 => (),
                    1 => {
                        horizontal = false;
                        reverse = true;
                    }
                    2 => {
                        i = 9 - i;
                        reverse = true;
                    }
                    3 => {
                        horizontal = false;
                        i = 9 - i;
                    }
                    _ => panic!(),
                }
                if cell.unwrap().flipped {
                    if horizontal {
                        reverse = !reverse;
                    } else {
                        i = 9 - i;
                    }
                }

                if horizontal {
                    if reverse {
                        result_row.extend(cell.unwrap().data[i].iter().rev().map(bool_map));
                    } else {
                        result_row.extend(cell.unwrap().data[i].iter().map(bool_map));
                    }
                } else {
                    if reverse {
                        result_row.extend(cell.unwrap().data.iter().rev().map(|x| bool_map(&x[i])));
                    } else {
                        result_row.extend(cell.unwrap().data.iter().map(|x| bool_map(&x[i])));
                    }
                }
            }
            result.push(result_row);
        }
    }

    result
}

fn bool_map(c: &char) -> bool {
    match c {
        '#' => true,
        '.' => false,
        _ => panic!(),
    }
}
