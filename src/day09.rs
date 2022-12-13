use std::{collections::HashSet, str::SplitAsciiWhitespace, cmp::max};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
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

    fn rope(self, head: &Pos2d) -> Pos2d {
        let dx = self.x - head.x;
        let dy = self.y - head.y;

        if max(dy.abs(),dx.abs()) < 2 {
            return self;
        } 
        
        match (dx.signum(), dy.signum()) {
           (0,0) => self,

           (-1,0) => self.right(),
           (1,0) => self.left(),

           (0,-1) => self.down(),
           (0,1) => self.up(),

           (1,1) => self.left().up(),
           (-1,1) => self.right().up(),

           (1,-1) => self.left().down(),
           (-1,-1) => self.right().down(),

           _ => todo!(),
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

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl From<SplitAsciiWhitespace<'_>> for Instruction {
    fn from(mut split: SplitAsciiWhitespace) -> Self {
        match split.next().unwrap().chars().next().unwrap() {
            'U' => Instruction::Up(split.next().unwrap().parse().unwrap()),
            'D' => Instruction::Down(split.next().unwrap().parse().unwrap()),
            'L' => Instruction::Left(split.next().unwrap().parse().unwrap()),
            'R' => Instruction::Right(split.next().unwrap().parse().unwrap()),
            _ => todo!(),
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut positions: HashSet<Pos2d> = HashSet::default();

    let mut head = Pos2d::default();
    let mut tails = [Pos2d::default(); 1];

    let mut update = |op: fn(Pos2d) -> Pos2d| {
        head = op(head);
        for tail in tails.iter_mut() {
            *tail = tail.rope(&head);
        }
        positions.insert(*tails.last().unwrap());
    };

    for instruction in input
        .lines()
        .map(|line| line.split_ascii_whitespace().into())
    {
        dbg!(instruction);
        match instruction {
            Instruction::Right(amount) => (0..amount).for_each(|_| { update(Pos2d::right); }),
            Instruction::Left(amount)  => (0..amount).for_each(|_| { update(Pos2d::left); }),
            Instruction::Up(amount)    => (0..amount).for_each(|_| { update(Pos2d::up); }),
            Instruction::Down(amount)  => (0..amount).for_each(|_| { update(Pos2d::down); }),
        }
    }

    for y in -10..10 {
        for x in -10..10 {
            print!("{}", if positions.contains(&Pos2d{x,y}) {"#"} else {" "});
        }
        println!();
    }

    positions.len() as i32
}

fn part2(input: &str) -> i32 {
    let mut positions: HashSet<Pos2d> = HashSet::default();

    let mut head = Pos2d::default();
    let mut tails = [Pos2d::default(); 9];

    let mut update = |op: fn(Pos2d) -> Pos2d| {
        head = op(head);
        let mut prev_tail = head;
        for tail in tails.iter_mut() {
            *tail = tail.rope(&prev_tail);
            prev_tail = *tail;
        }
        positions.insert(*tails.last().unwrap());
    };

    for instruction in input
        .lines()
        .map(|line| line.split_ascii_whitespace().into())
    {
        dbg!(instruction);
        match instruction {
            Instruction::Right(amount) => (0..amount).for_each(|_| { update(Pos2d::right); }),
            Instruction::Left(amount)  => (0..amount).for_each(|_| { update(Pos2d::left); }),
            Instruction::Up(amount)    => (0..amount).for_each(|_| { update(Pos2d::up); }),
            Instruction::Down(amount)  => (0..amount).for_each(|_| { update(Pos2d::down); }),
        }
    }

    // for y in -30..30 {
    //     for x in -30..30 {
    //         print!("{}", if positions.contains(&Pos2d{x,y}) {"#"} else {" "});
    //     }
    //     println!();
    // }

    positions.len() as i32
}
#[test]
fn test() {
    let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 1);
}

#[test]
fn real() {
    let input = include_str!("../input/day09.txt");
    assert_eq!(part1(input), 6271);
    assert_eq!(part2(input), 2458);
}
