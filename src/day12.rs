use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};

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
    cost: usize,
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
        other.cost.cmp(&self.cost)
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
fn shortest_path(map: &str, start: Pos2d, goal: Pos2d) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<Pos2d, usize> = Default::default();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if dist.contains_key(&position) && cost > *dist.get(&position).unwrap() { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in edges(map, position) {
            let next = State { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if !dist.contains_key(&next.position) || next.cost < *dist.get(&next.position).unwrap() {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist.insert(next.position, next.cost);
            }
        }
    }

    // Goal not reachable
    None
}
