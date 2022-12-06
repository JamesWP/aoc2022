use std::ops::RangeInclusive;

fn parse_pairs(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let parse_range = |pair: &str| {
        let mut item = pair.split("-").map(str::parse).map(Result::unwrap);

        let from = item.next().unwrap();
        let to = item.next().unwrap();

        from..=to
    };
    let mut pairs = line.split(",").map(parse_range);

    let a = pairs.next().unwrap();
    let b = pairs.next().unwrap();

    (a, b)
}

fn includes(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    a.start() <= b.start() && a.end() >= b.end()
}

fn overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    (b.start() <= a.start() && a.start() <= b.end()) || (b.start() <= a.end() && a.end() <= b.end())
}

fn either_overlaps(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    overlaps(a, b) || overlaps(b, a)
}

fn either_includes(a: &RangeInclusive<u32>, b: &RangeInclusive<u32>) -> bool {
    includes(a, b) || includes(b, a)
}

fn part1(input: &str) -> u32 {
    let count_of_overlaps = input
        .split("\n")
        .map(parse_pairs)
        .filter(|(a, b)| either_includes(a, b))
        .count();

    count_of_overlaps as u32
}

fn part2(input: &str) -> u32 {
    let count_of_any_overlap = input
        .split("\n")
        .map(parse_pairs)
        .filter(|(a, b)| either_overlaps(a, b))
        .count();

    count_of_any_overlap as u32
}

#[test]
fn test() {
    let pairs = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    assert_eq!(2, part1(pairs));
    assert_eq!(4, part2(pairs));
}

#[test]
fn input() {
    let input = std::fs::read_to_string("input/day04.txt").unwrap();

    assert_eq!(513, part1(&input));
    assert_eq!(878, part2(&input));
}
