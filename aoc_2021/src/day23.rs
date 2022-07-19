use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

const POSITIONS: usize = 4;

#[derive(Eq, PartialEq)]
struct State {
    energy: usize,
    rooms: [[Option<usize>; POSITIONS]; 4],
    hallway: [Option<usize>; 11],
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.energy.cmp(&self.energy)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn hash(&self) -> u128 {
        let mut value = 0u128;
        for hallway_pos in [0, 1, 3, 5, 7, 9, 10] {
            if let Some(t) = self.hallway[hallway_pos] {
                value += (t + 1) as u128;
            }
            value = value.checked_shl(3).unwrap();
        }
        for room in 0..4 {
            for pos in 0..POSITIONS {
                if let Some(t) = self.rooms[room][pos] {
                    value += (t + 1) as u128;
                }
                value = value.checked_shl(3).unwrap();
            }
        }
        value
    }

    fn print(&self) {
        println!(
            "{} energy: {}",
            self.hallway
                .iter()
                .map(|x| if x.is_some() {
                    x.unwrap().to_string()
                } else {
                    ".".to_string()
                })
                .collect::<String>(),
            self.energy
        );
        println!(
            "  {}",
            self.rooms
                .iter()
                .map(|x| if x[0].is_some() {
                    x[0].unwrap().to_string()
                } else {
                    ".".to_string()
                } + " ")
                .collect::<String>()
        );
        println!(
            "  {}",
            self.rooms
                .iter()
                .map(|x| if x[1].is_some() {
                    x[1].unwrap().to_string()
                } else {
                    ".".to_string()
                } + " ")
                .collect::<String>()
        );
    }
}

pub fn task1(input: &str) {
    let mut result = 0usize;

    let mut rooms = [[None; POSITIONS]; 4];
    for (pos, line) in input.lines().skip(2).take(POSITIONS).enumerate() {
        for room in 0..4 {
            rooms[room][pos] = Some((line.as_bytes()[3 + room * 2] - 65) as usize);
        }
    }

    let hallway = [None; 11];

    let mut states = BinaryHeap::new();
    states.push(State {
        energy: 0,
        rooms,
        hallway,
    });
    let mut hashes = HashMap::new();

    while let Some(state) = states.pop() {
        if state
            .rooms
            .iter()
            .enumerate()
            .all(|x| x.1.iter().all(|y| *y == Some(x.0)))
        {
            println!("Found result");
            result = state.energy;
            break;
        }
        for hallway_pos in [0, 1, 3, 5, 7, 9, 10] {
            for room in 0..4 {
                for pos in 0..POSITIONS {
                    if let Some(new_state) = move_to_hallway(room, pos, hallway_pos, &state) {
                        let hash = new_state.hash();
                        if !hashes.contains_key(&hash)
                            || *hashes.get(&hash).unwrap() > new_state.energy
                        {
                            hashes.insert(hash, new_state.energy);
                            states.push(new_state);
                        }
                    }
                }
                if let Some(new_state) = move_to_room(hallway_pos, room, &state) {
                    let hash = new_state.hash();
                    if !hashes.contains_key(&hash) || *hashes.get(&hash).unwrap() > new_state.energy
                    {
                        hashes.insert(hash, new_state.energy);
                        states.push(new_state);
                    }
                }
            }
        }
    }

    println!("Result: {}", result);
}

fn cost(t: usize) -> usize {
    match t {
        0 => 1,
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!(),
    }
}

fn diff(a: usize, b: usize) -> usize {
    if a >= b {
        a - b
    } else {
        b - a
    }
}

fn move_to_hallway(room: usize, pos: usize, hallway_pos: usize, state: &State) -> Option<State> {
    if state.rooms[room][pos].is_none() {
        return None;
    }
    let t = state.rooms[room][pos].unwrap();
    if pos > 0 && state.rooms[room].iter().take(pos).any(|x| x.is_some()) {
        return None;
    }
    let mut energy = pos + 1;

    let room_hallway_pos = 2 + room * 2;
    energy += diff(hallway_pos, room_hallway_pos);

    for i in room_hallway_pos.min(hallway_pos)..=room_hallway_pos.max(hallway_pos) {
        if state.hallway[i].is_some() {
            return None;
        }
    }

    let mut new_state = State {
        energy: state.energy + energy * cost(t),
        rooms: state.rooms.clone(),
        hallway: state.hallway.clone(),
    };
    new_state.hallway[hallway_pos] = new_state.rooms[room][pos];
    new_state.rooms[room][pos] = None;
    Some(new_state)
}

fn move_to_room(hallway_pos: usize, room: usize, state: &State) -> Option<State> {
    if state.hallway[hallway_pos].is_none() {
        return None;
    }
    let t = state.hallway[hallway_pos].unwrap();
    if room != t
        || state.rooms[room]
            .iter()
            .any(|x| x.is_some() && x.unwrap() != room)
    {
        return None;
    }
    let pos = state.rooms[room]
        .iter()
        .position(|x| x.is_some())
        .unwrap_or(POSITIONS)
        - 1;
    let mut energy = pos + 1;

    let room_hallway_pos = 2 + room * 2;
    energy += diff(hallway_pos, room_hallway_pos);

    for i in room_hallway_pos.min(hallway_pos)..=room_hallway_pos.max(hallway_pos) {
        if i != hallway_pos && state.hallway[i].is_some() {
            return None;
        }
    }

    let mut new_state = State {
        energy: state.energy + energy * cost(t),
        rooms: state.rooms.clone(),
        hallway: state.hallway.clone(),
    };
    new_state.rooms[room][pos] = new_state.hallway[hallway_pos];
    new_state.hallway[hallway_pos] = None;
    Some(new_state)
}

pub fn task2(input: &str) {
    let mut result = 0usize;
    println!("Result: {}", result);
}
