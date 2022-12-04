#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Result {
    Win,
    Loose,
    Draw,
}

fn opponent(choice: char) -> Choice {
    match choice {
        'A' => Choice::Rock,
        'B' => Choice::Paper,
        'C' => Choice::Scissors,
        _ => panic!(),
    }
}

fn me(choice: char) -> Choice {
    match choice {
        'X' => Choice::Rock,
        'Y' => Choice::Paper,
        'Z' => Choice::Scissors,
        _ => panic!(),
    }
}

fn result(result:char) -> Result {
    match result {
        'X' => Result::Loose,
        'Y' => Result::Draw,
        'Z' => Result::Win,
        _ => panic!(),
    }
}

fn score(me: Choice, opponent: Choice) -> Result {
    match (me, opponent) {
        (Choice::Rock, Choice::Rock) => Result::Draw,
        (Choice::Rock, Choice::Paper) => Result::Loose,
        (Choice::Rock, Choice::Scissors) => Result::Win,
        (Choice::Paper, Choice::Rock) => Result::Win,
        (Choice::Paper, Choice::Paper) => Result::Draw,
        (Choice::Paper, Choice::Scissors) => Result::Loose,
        (Choice::Scissors, Choice::Rock) => Result::Loose,
        (Choice::Scissors, Choice::Paper) => Result::Win,
        (Choice::Scissors, Choice::Scissors) => Result::Draw,
    }
}

fn get_shape(me: Choice, outcome: Result) -> Choice {
    match (me, outcome) {
        (Choice::Rock, Result::Win) => Choice::Paper,
        (Choice::Rock, Result::Loose) => Choice::Scissors,
        (Choice::Rock, Result::Draw) => Choice::Rock,
        (Choice::Paper, Result::Win) => Choice::Scissors,
        (Choice::Paper, Result::Loose) => Choice::Rock,
        (Choice::Paper, Result::Draw) => Choice::Paper,
        (Choice::Scissors, Result::Win) => Choice::Rock,
        (Choice::Scissors, Result::Loose) => Choice::Paper,
        (Choice::Scissors, Result::Draw) => Choice::Scissors,
    }
}

fn round_score(me: Choice, opponent: Choice) -> u32 {
    let result_points = match score(me, opponent) {
        Result::Win => 6,
        Result::Loose => 0,
        Result::Draw => 3,
    };

    let shape_points = match me {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    };

    shape_points + result_points
}

fn total_score(input: &str) -> u32 {
    let total_score: u32 = input
        .split("\n")
        .map(|line| {
            let opponent_shape = line.split(" ").nth(0).unwrap();
            let me_shape = line.split(" ").nth(1).unwrap();

            let opponent_shape = opponent(opponent_shape.chars().nth(0).unwrap());
            let me_shape = me(me_shape.chars().nth(0).unwrap());

            round_score(me_shape, opponent_shape)
        })
        .sum();

    total_score
}

fn new_total_score(input: &str) -> u32 {
    let total_score: u32 = input
        .split("\n")
        .map(|line| {
            let opponent_shape = line.split(" ").nth(0).unwrap();
            let desired_outcome = line.split(" ").nth(1).unwrap();

            let opponent_shape = opponent(opponent_shape.chars().nth(0).unwrap());
            let desired_outcome = result(desired_outcome.chars().nth(0).unwrap());

            round_score(get_shape(opponent_shape, desired_outcome), opponent_shape)
        })
        .sum();

    total_score
}

#[test]
fn test_part_01() {
    let test_input = "A Y
B X
C Z";

    assert_eq!(15, total_score(test_input));
    assert_eq!(12, new_total_score(test_input));
}

#[test]
fn test_part_01_real() {
    let input = std::fs::read_to_string("input/day02.txt").unwrap();

    assert_eq!(11906, total_score(&input));
    assert_eq!(0, new_total_score(&input));
}