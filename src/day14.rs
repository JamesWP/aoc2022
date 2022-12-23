use std::{collections::HashSet, sync::mpsc::Receiver};

use crate::pos2d::Pos2d;

#[derive(PartialEq, Clone, Copy)]
enum RecursionDecision {
    Sentinal,
    Continue,
}

type LocationSet = HashSet<Pos2d>;

fn recursion(location: Pos2d, wl: &mut LocationSet, ssl: &mut LocationSet, pit_begins: i32, pit_decision: RecursionDecision) -> RecursionDecision {
    // (global) base case check. are we now in the bottomless pit?
    // -> return Sentinal.
    if location.y >= pit_begins {
        return pit_decision;
    }

    // base case check. are we now in a wall, or already contains sand?
    // -> return Continue.
    if wl.contains(&location) {
        return RecursionDecision::Continue;
    }
    if ssl.contains(&location) {
        return RecursionDecision::Continue;
    }

    // Recursion:
    // -> down
    //    if this returns Sentinal -> return
    // -> down left
    //    if this returns Sentinal -> return
    // -> down right
    //    if this returns Sentinal -> return
    if RecursionDecision::Sentinal == recursion(location.down(), wl, ssl, pit_begins, pit_decision) {
        return RecursionDecision::Sentinal;
    }
    if RecursionDecision::Sentinal == recursion(location.down().left(), wl, ssl, pit_begins, pit_decision) {
        return RecursionDecision::Sentinal;
    }
    if RecursionDecision::Sentinal == recursion(location.down().right(), wl, ssl, pit_begins, pit_decision) {
        return RecursionDecision::Sentinal;
    }

    // Place sand at this location in the ssl set.
    ssl.insert(location);

    RecursionDecision::Continue
}

fn part1(input: &str) -> i32 {
    solve(input, RecursionDecision::Sentinal)
}

fn part2(input: &str) -> i32 {
    solve(input, RecursionDecision::Continue)
}

fn solve(input: &str, pit_decision: RecursionDecision) -> i32 {
    // Parse input into wall locations set (wl)
    let mut wl = LocationSet::default();

    for line in input.lines() {
        let mut parts = line.split("->");
        let mut pos = parts.next().unwrap().into();

        for part in parts {
            let part = part.into();

            wl.insert(pos);
            while pos != part {
                pos = pos.move_in_direction(pos.cmp_no_diagonal(&part));
                wl.insert(pos);
            }
        }
    }

    // Create empty stationary sand locations set (ssl)
    let mut ssl = LocationSet::default();

    // calculate depth of bottomless pit +1
    let pit_begins = 2 + wl.iter().map(|p| p.y).max().unwrap();

    recursion(Pos2d { x: 500, y: 0 }, &mut wl, &mut ssl, pit_begins, pit_decision);

    ssl.len() as i32
}

#[test]
fn test() {
    let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    assert_eq!(part1(input), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn real() {
    let input = include_str!("../input/day14.txt");

    assert_eq!(part1(input), 1003);
    assert_eq!(part2(input), 25771);
}
