use std::ops::Deref;

fn get_message(stacks: &[Vec<char>], input: &str, part_two: bool) -> String {
    let mut stacks = stacks.to_owned();

    stacks.iter_mut().for_each(|stack| stack.reverse());

    input.split("\n").for_each(|mov|{
        let mut parts = mov.split(" ");
        parts.next();
        let number: u32 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let source_stack: u32 = parts.next().unwrap().parse().unwrap();
        parts.next();
        let destination_stack: u32 = parts.next().unwrap().parse().unwrap();

        let mut crates: Vec<char> = (0..number).map(|_i| {
            stacks.get_mut((source_stack-1) as usize).unwrap().pop().unwrap()
        }).collect();

        if part_two {
            crates.reverse();
        }

        stacks.get_mut((destination_stack-1) as usize).unwrap().append(&mut crates);
    });

    let result:String = stacks.iter().map(|stack| stack.last().unwrap()).collect();
    
    result
}

#[test]
fn test() {
    let stacks = [vec!['N', 'Z'], vec!['D', 'C', 'M'], vec!['P']];

    let input = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    assert_eq!("CMZ", get_message(&stacks, input, false));
    assert_eq!("MCD", get_message(&stacks, input, true));
}

#[test]
fn real() {
    let stacks = [
        vec!['D','Z','T','H'],
        vec!['S','C','G','T','W','R','Q'],
        vec!['H','C','R','N','Q','F','B','P'],
        vec!['Z','H','F','N','C','L'],
        vec!['S','Q','F','L','G'],
        vec!['S','C','R','B','Z','W','P','V'],
        vec!['J','F','Z'],
        vec!['Q','H','R','Z','V','L','D'],
        vec!['D','L','Z','F','N','G','H','B'],
    ];

    let input = std::fs::read_to_string("input/day05.txt").unwrap();

    assert_eq!("RFFFWBPNS", get_message(&stacks, &input, false));
    assert_eq!("CQQBBJFCS", get_message(&stacks, &input, true));
}
