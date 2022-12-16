use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug, Ord, PartialOrd)]
struct Pos2d {
    x: i32,
    y: i32,
}

impl Pos2d {
    fn up(self) -> Pos2d {
        Pos2d {
            x: self.x,
            y: self.y - 1,
        }
    }
    fn down(self) -> Pos2d {
        Pos2d {
            x: self.x,
            y: self.y + 1,
        }
    }
    fn left(self) -> Pos2d {
        Pos2d {
            x: self.x - 1,
            y: self.y,
        }
    }
    fn right(self) -> Pos2d {
        Pos2d {
            x: self.x + 1,
            y: self.y,
        }
    }
}

impl Default for Pos2d {
    fn default() -> Self {
        Self {
            x: Default::default(),
            y: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: i32,
    position: Pos2d,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(map: &[u8], size: (usize, usize), start: &[Pos2d], goal: Pos2d) -> Option<i32> {

    // dbg!(start, goal);

    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Pos2d, i32> = Default::default();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    for start in start {
        dist.insert(*start, 0);
        heap.push(State {
            cost: 0,
            position: *start,
        });
    }

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if dist.contains_key(&position) && cost > *dist.get(&position).unwrap() {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in edges(map, size, &position) {
            let next = State {
                cost: cost + 1,
                position: edge,
            };

            // If so, add it to the frontier and continue
            if !dist.contains_key(&next.position) || next.cost < *dist.get(&next.position).unwrap()
            {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }

    for y in 0..size.1 as i32 {
        for x in 0..size.0 as i32 {
            print!("{}", dist.get(&Pos2d{x,y}).unwrap_or(&0)%10);
        }
        println!();
    }

    // Goal not reachable
    None
}

fn get(map: &[u8], size: (usize, usize), position: &Pos2d) -> (u8,char) {
    let size = size.0 as i32;
    let &c = map
        .get((position.y * (size + 1) + position.x) as usize)
        .unwrap();
    if c == 'S' as u8 {
        (0, 'S')
    } else if c == 'E' as u8 {
        (26, 'E')
    } else {
        (c - 'a' as u8, '\0')
    }
}

fn edges(map: &[u8], size: (usize, usize), position: &Pos2d) -> Vec<Pos2d> {
    [
        position.left(),
        position.right(),
        position.down(),
        position.up(),
    ]
    .into_iter()
    .filter(|p| p.x >= 0 && p.y >= 0 && p.y < size.1 as i32 && p.x < size.0 as i32)
    .filter(|p| (get(map, size, &p).0) <= get(map, size, position).0+1)
    .collect()
}

fn part1(input: &str, size: (usize, usize)) -> i32 {

    let map = input.as_bytes();
    let mut start = Pos2d::default();
    let mut goal = Pos2d::default();

    for x in 0..size.0 as i32 {
        for y in 0..size.1 as i32 {
            if get(map, size, &Pos2d { x, y }).1 == 'S' {
                start = Pos2d { x, y };
            } else if get(map, size, &Pos2d { x, y }).1 == 'E' {
                goal = Pos2d { x, y };
            }
        }
    }

    shortest_path(map, size, &[start], goal).unwrap()
} 
fn part2(input: &str, size: (usize, usize)) -> i32 {

    let map = input.as_bytes();
    let mut start = Vec::default();
    let mut goal = Pos2d::default();

    for x in 0..size.0 as i32 {
        for y in 0..size.1 as i32 {
            if get(map, size, &Pos2d { x, y }).1 == 'S' {
                start.push(Pos2d{ x, y });
            } else if get(map, size, &Pos2d { x, y }).0 == 0 {
                start.push(Pos2d{ x, y });
            } else if get(map, size, &Pos2d { x, y }).1 == 'E' {
                goal = Pos2d { x, y };
            }
        }
    }

    shortest_path(map, size, &start, goal).unwrap()
} 
#[test]
fn test() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let size = (8,5);

    assert_eq!(part1(input, size), 31);
    assert_eq!(part2(input, size), 29);
}

#[test]
fn real() {
    let input = include_str!("../input/day12.txt");
    let size = (64, 41);

    assert_eq!(part1(input, size), 370);
    assert_eq!(part2(input, size), 363);
}