#![feature(iter_next_chunk)]
use std::sync::Arc;

fn split_in_two(input: &str) -> (&str, &str) {
    let size = input.len();
    assert!(size % 2 == 0);
    let size = size / 2;

    input.split_at(size)
}

fn char_value(mut c: char) -> u32 {
    let mut value = 0;
    if c.is_uppercase() {
        value += 26;
    }
    c.make_ascii_uppercase();

    let c = c as u32;
    let a = 'A' as u32;
    let z = 'Z' as u32;

    assert!(a <= c);
    assert!(c <= z);

    value += 1;

    let c = c - ('A' as u32);

    value += c;

    value
}

fn value_char(mut value: u32) -> char {
    let uppercase = value >= 27;

    if uppercase {
        value -= 26;
    }

    let a = 'A' as u32;
    let z = 'Z' as u32;

    value -= 1;

    value += a;

    assert!(value >= a);
    assert!(z >= value);

    let mut value = std::char::from_u32(value).unwrap();

    if !uppercase {
        value = value.to_ascii_lowercase();
    }

    value
}

#[test]
fn test_char_values() {
    assert_eq!(16, char_value('p'));
    assert_eq!(38, char_value('L'));
    assert_eq!(42, char_value('P'));
    assert_eq!(20, char_value('t'));
    assert_eq!(19, char_value('s'));

    assert_eq!(value_char(16), 'p');
    assert_eq!(value_char(38), 'L');
    assert_eq!(value_char(42), 'P');
    assert_eq!(value_char(20), 't');
    assert_eq!(value_char(19), 's');
}

fn get_first_in_both(inputs: (&str, &str)) -> u32 {
    let (a, b) = inputs;
    let mut charset = [0; 26 * 2];

    for c in a.chars() {
        let c = char_value(c);
        charset[(c - 1) as usize] += 1;
    }

    for c in b.chars() {
        let c = char_value(c);
        if charset[(c - 1) as usize] > 0 {
            return c;
        }
    }

    panic!();
}

fn rucksacks(input: &str) -> u32 {
    let sum: u32 = input
        .split("\n")
        .map(split_in_two)
        .map(get_first_in_both)
        .sum();
    sum
}

fn groups(input: &str) -> u32 {
    let mut bags = input.split("\n");
    let mut answer = 0;

    'elf_group: loop {
        let elf_a = if let Some(b) = bags.next() {
            b
        } else {
            return answer;
        };
        let elf_b = bags.next().unwrap();
        let elf_c = bags.next().unwrap();

        let mut charset = [0; 26 * 2];
        
        for c in elf_a.chars() {
            let c = char_value(c);
            charset[(c - 1) as usize] |= 0b001;
        }
        for c in elf_b.chars() {
            let c = char_value(c);
            charset[(c - 1) as usize] |= 0b010;
        }
        for c in elf_c.chars() {
            let c = char_value(c);
            charset[(c - 1) as usize] |= 0b100;
        }

        for i in 1..=52 {
            if charset[i - 1 as usize] == 0b111 {
                answer += i as u32;
                continue 'elf_group;
            }
        }

        panic!("no item found");
    }
}

#[test]
fn test_part_01() {
    let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    assert_eq!(157, rucksacks(input));
    assert_eq!(70, groups(input));
}

#[test]
fn test_part_01_real() {
    let input = std::fs::read_to_string("input/day03.txt").unwrap();

    assert_eq!(8394, rucksacks(&input));
    assert_eq!(0, groups(&input));
}
