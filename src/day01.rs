fn solve(input: &str) -> (u64, i32) {
    let max = input.split("\n\n").zip(1..).map(|(inventory, elf_number)|{
        let total_calorites = inventory.split("\n").map(|item| item.parse::<u64>().unwrap()).sum::<u64>();

        (total_calorites, elf_number)
    }).max_by(|a,b| a.0.cmp(&b.0));

    max.unwrap()
}

fn solve_02(input: &str) -> u64 {
    let mut calories :Vec<_> = input.split("\n\n").map(|inventory|{
        let total_calorites = inventory.split("\n").map(|item| item.parse::<u64>().unwrap()).sum::<u64>();

        total_calorites
    }).collect();

    let pivot = calories.len() -3;
    calories.select_nth_unstable(pivot);

    let top_three = &mut calories.as_mut_slice()[pivot..];

    top_three.sort();

    top_three.iter().sum()
}

const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

#[test]
fn day_01_part_01() {
    assert_eq!(4, solve(TEST_INPUT).1);
}

#[test]
fn day_01_part_01_real() {
    let data = std::fs::read_to_string(format!("input/day{:02}.txt", 1)).unwrap();

    assert_eq!(70116, solve(data.as_str()).0);
}

#[test]
fn day_01_part_02() {
    assert_eq!(45000, solve_02(TEST_INPUT));
}

#[test]
fn day_01_part_02_real() {
    let data = std::fs::read_to_string(format!("input/day{:02}.txt", 1)).unwrap();

    assert_eq!(0, solve_02(data.as_str()));
}