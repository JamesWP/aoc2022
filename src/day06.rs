
fn has_repetition(input: &str) -> bool {
    let mut charset = [0;26];

    input.chars().for_each(|c| charset[c as usize - ('a' as usize)] += 1);

    charset.iter().filter(|v|**v > 1).next().is_some()
}

#[test]
fn test_has_rep() {
    assert!(has_repetition("thisstringhasrep"));
    assert!(!has_repetition("abcd"));
}

fn part1(input: &str) -> u32 {
    let windows = 0..(input.len()-4);
    
    let windows = windows.map(|start| start..start+4);

    let windows = windows.map(|range| &input[range]);

    let result = windows.zip(4..).filter(|(input, _idx)|!has_repetition(*input)).take(1).next().unwrap();

    result.1
}

fn part2(input: &str) -> u32 {
    let windows = 0..(input.len()-14);
    
    let windows = windows.map(|start| start..start+14);

    let windows = windows.map(|range| &input[range]);

    let result = windows.zip(14..).filter(|(input, _idx)|!has_repetition(*input)).take(1).next().unwrap();

    result.1
}

#[test]
fn test() {
    assert_eq!(7, part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, part1("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, part1("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));

    assert_eq!(19, part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, part2("bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, part2("nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}
#[test]
fn real() {
    let input = include_str!("../input/day06.txt");

    assert_eq!(1647, part1(input));
    assert_eq!(2447, part2(input));
}