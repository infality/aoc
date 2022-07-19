use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Eq, PartialEq)]
pub struct HeapNode {
    pub pos: (usize, usize),
    pub distance: usize,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &HeapNode) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &HeapNode) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

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

    let size = data.len();
    let mut queue = BinaryHeap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push(HeapNode {
        pos: (0, 0),
        distance: 0,
    });
    distances.insert((0, 0), 0);

    while let Some(node) = queue.pop() {
        if node.pos == (size - 1, size - 1) {
            result = distances[&(size - 1, size - 1)];
            break;
        }

        let mut neighbors = Vec::new();
        if node.pos.0 > 0 {
            neighbors.push((node.pos.0 - 1, node.pos.1));
        }
        if node.pos.0 < size - 1 {
            neighbors.push((node.pos.0 + 1, node.pos.1));
        }
        if node.pos.1 > 0 {
            neighbors.push((node.pos.0, node.pos.1 - 1));
        }
        if node.pos.1 < size - 1 {
            neighbors.push((node.pos.0, node.pos.1 + 1));
        }

        for neighbor in neighbors.iter() {
            let dist = data[neighbor.0][neighbor.1];
            let new_distance = distances[&node.pos] + dist;

            if !distances.contains_key(neighbor) || new_distance < distances[neighbor] {
                queue.push(HeapNode {
                    pos: (neighbor.0, neighbor.1),
                    distance: new_distance,
                });
                distances.insert(*neighbor, new_distance);
                prev.insert(*neighbor, node.pos);
            }
        }
    }

    println!("Result: {}", result);
}

pub fn task2(input: &str) {
    let mut result = 0usize;

    let mut data = Vec::new();
    for line in input.split('\n') {
        let mut row = Vec::new();
        for entry in line.chars() {
            row.push(entry.to_string().parse::<usize>().unwrap());
        }
        data.push(row);
    }

    let size = data.len();
    let mut queue = BinaryHeap::new();
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut prev: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    queue.push(HeapNode {
        pos: (0, 0),
        distance: 0,
    });
    distances.insert((0, 0), 0);

    while let Some(node) = queue.pop() {
        if node.pos == (5 * size - 1, 5 * size - 1) {
            result = distances[&(5 * size - 1, 5 * size - 1)];
            break;
        }

        let mut neighbors = Vec::new();
        if node.pos.0 > 0 {
            neighbors.push((node.pos.0 - 1, node.pos.1));
        }
        if node.pos.0 < 5 * size - 1 {
            neighbors.push((node.pos.0 + 1, node.pos.1));
        }
        if node.pos.1 > 0 {
            neighbors.push((node.pos.0, node.pos.1 - 1));
        }
        if node.pos.1 < 5 * size - 1 {
            neighbors.push((node.pos.0, node.pos.1 + 1));
        }

        for neighbor in neighbors.iter() {
            let additional_dist = neighbor.0 / size + neighbor.1 / size;
            let mut dist = data[neighbor.0 % size][neighbor.1 % size] + additional_dist;
            if dist > 9 {
                dist = (dist % 10) + 1;
            }
            let new_distance = distances[&node.pos] + dist;

            if !distances.contains_key(neighbor) || new_distance < distances[neighbor] {
                queue.push(HeapNode {
                    pos: (neighbor.0, neighbor.1),
                    distance: new_distance,
                });
                distances.insert(*neighbor, new_distance);
                prev.insert(*neighbor, node.pos);
            }
        }
    }

    println!("Result: {}", result);
}
