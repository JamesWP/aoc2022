use std::collections::HashSet;

fn part2_compute_scenic_score(input: &[u8], size: i32, x: i32, y: i32) -> i32 {
    part2_compute_view_distance(input, size, x, y, 1, 0)
        * part2_compute_view_distance(input, size, x, y, 0, 1)
        * part2_compute_view_distance(input, size, x, y, -1, 0)
        * part2_compute_view_distance(input, size, x, y, 0, -1)
}

fn part2_compute_view_distance(input: &[u8], size: i32, x: i32, y: i32, dx: i32, dy: i32) -> i32 {
    let get = |x, y| (input.get((y * (size + 1) + x) as usize).unwrap() - '0' as u8) as i32;

    let my_height = get(x, y);
    for step in 1..=(size as i32) {
        let next_x = x + dx * step;
        if next_x < 0 || next_x >= size {
            return step-1;
        }
        let next_y = y + dy * step;
        if next_y < 0 || next_y >= size {
            return step-1;
        }
        if my_height <= get(next_x, next_y) {
            return step;
        }
    }
    panic!();
}
fn part2(input: &str, size: i32) -> i32{
    let input = input.as_bytes();

    let mut max_score = 0;
    for x in 0..size {
        for y in 0..size {
            let score = part2_compute_scenic_score(input, size, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn part1(input: &str, size: usize) -> u32 {
    let input = input.as_bytes();
    let get = |x, y| (input.get(y * (size + 1) + x).unwrap() - '0' as u8) as i32;

    let mut set: HashSet<(usize, usize)> = HashSet::new();

    for y in 0..size {
        let mut height = -1;
        for x in 0..size {
            if get(x, y) > height {
                height = get(x, y);
                set.insert((x, y));
            }
            if height == 9 {
                break;
            }
        }
    }
    for y in 0..size {
        let mut height = -1;
        for x in (0..size).rev() {
            if get(x, y) > height {
                height = get(x, y);
                set.insert((x, y));
            }
            if height == 9 {
                break;
            }
        }
    }
    for x in 0..size {
        let mut height = -1;
        for y in 0..size {
            if get(x, y) > height {
                height = get(x, y);
                set.insert((x, y));
            }
            if height == 9 {
                break;
            }
        }
    }
    for x in 0..size {
        let mut height = -1;
        for y in (0..size).rev() {
            if get(x, y) > height {
                height = get(x, y);
                set.insert((x, y));
            }
            if height == 9 {
                break;
            }
        }
    }

    // for y in 0..size {
    //     for x in 0..size {
    //         print!("{}", if set.contains(&(x,y)) {"X"} else {"#"});
    //     }
    //     println!();
    // }
    set.len().try_into().unwrap()
}

#[test]
fn test() {
    let input = "30373
25512
65332
33549
35390";

    let tree_count = part1(input, 5);
    assert_eq!(21, tree_count);

    {
        let input = input.as_bytes();

        assert_eq!(part2_compute_view_distance(input, 5, 2, 1, 0, -1), 1);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 1, -1, 0), 1);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 1, 1, 0), 2);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 1, 0, 1), 2);

        assert_eq!(part2_compute_view_distance(input, 5, 2, 3, 0, -1), 2);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 3, -1, 0), 2);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 3, 1, 0), 2);
        assert_eq!(part2_compute_view_distance(input, 5, 2, 3, 0, 1), 1);

        assert_eq!(part2_compute_scenic_score(input, 5, 2, 1), 4);
    }

    assert_eq!(part2(input, 5), 8);
}

#[test]
fn real() {
    let input = include_str!("../input/day08.txt");

    let tree_count = part1(input, 99);
    assert_eq!(1763, tree_count);
    assert_eq!(part2(input, 99), 671160);
}
