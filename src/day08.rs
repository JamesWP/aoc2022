use std::collections::HashSet;

fn part1(input: &str, size: usize) -> u32 {
    let input = input.as_bytes();
    let get = |x,y| { (input.get(y * (size+1) + x).unwrap() - '0' as u8) as i32 };

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..size { let mut height = -1; for x in 0..size         { if get(x,y) > height { height = get(x,y); set.insert((x,y)); } if height == 9 { break; } } }
    for y in 0..size { let mut height = -1; for x in (0..size).rev() { if get(x,y) > height { height = get(x,y); set.insert((x,y)); } if height == 9 { break; } } }
    for x in 0..size { let mut height = -1; for y in 0..size         { if get(x,y) > height { height = get(x,y); set.insert((x,y)); } if height == 9 { break; } } }
    for x in 0..size { let mut height = -1; for y in (0..size).rev() { if get(x,y) > height { height = get(x,y); set.insert((x,y)); } if height == 9 { break; } } }

    // for y in 0..size {
    //     for x in 0..size {
    //         print!("{}", if set.contains(&(x,y)) {"X"} else {"#"});
    //     }
    //     println!();
    // }
    set.len().try_into().unwrap()
}

#[test]
fn test(){

    let input = "30373
25512
65332
33549
35390";

    let tree_count = part1(input, 5);
    assert_eq!(21, tree_count);
}

#[test]
fn real() {
    let input = include_str!("../input/day08.txt");

    let tree_count = part1(input, 99);
    assert_eq!(0, tree_count);
}