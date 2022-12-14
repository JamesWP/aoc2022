use std::{
    cmp::Ordering::{Equal, Greater, Less},
    iter::Peekable,
    str::Chars,
};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Value {
    Literal(i32),
    List(Vec<Value>),
}

impl<'a> From<&mut Peekable<Chars<'a>>> for Value {
    fn from(input: &mut Peekable<Chars<'a>>) -> Self {
        let char = input.next().unwrap();
        match char {
            '[' => {
                let mut values = vec![];
                loop {
                    if input.peek() == Some(&']') {
                        break;
                    }
                    let value: Value = input.into();
                    values.push(value);

                    let next_char = input.next().unwrap();

                    match next_char {
                        ']' => break,
                        ',' => continue,
                        _ => todo!(),
                    };
                }

                Value::List(values)
            }
            '0'..='9' => {
                let mut number = Vec::new();
                number.push(char);
                loop {
                    if input.peek().is_none() {
                        break;
                    }

                    let &next = input.peek().unwrap();
                    if next.is_numeric() {
                        input.next().unwrap();
                        number.push(next);
                        continue;
                    }
                    break;
                }
                let number = String::from_iter(number.into_iter());
                Value::Literal(number.parse().unwrap())
            }
            _ => todo!(),
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        dbg!(value);
        let mut input = value.chars().peekable();
        Value::from(&mut input)
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // dbg!(self, other);
        match (self, other) {
            (Value::Literal(a), Value::Literal(b)) => a.cmp(b),
            (Value::Literal(l), Value::List(_)) => Value::List(vec![Value::Literal(*l)]).cmp(other),
            (Value::List(_), Value::Literal(l)) => self.cmp(&Value::List(vec![Value::Literal(*l)])),
            (Value::List(left), Value::List(right)) => {
                for (left, right) in left.iter().zip(right.iter()) {
                    match left.cmp(right) {
                        Equal => continue,
                        a => return a,
                    };
                }
                left.len().cmp(&right.len())
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> i32 {
    let mut count = 0;

    for (pair, index) in input.split("\n\n").zip(1..) {
        let mut parts = pair.split("\n");
        dbg!("comparing", index);
        let left:Value = parts.next().unwrap().into();
        let right:Value = parts.next().unwrap().into();

        dbg!("right order", left <= right);
        if left <= right {
            count+= index;
        }
    }

    count
}

#[test]
fn test() {
    assert_eq!(Value::Literal(1), "1".into());
    assert_eq!(Value::List(vec![Value::Literal(1)]), "[1]".into());
    assert_eq!(
        Value::List(vec![Value::Literal(1), Value::Literal(1)]),
        "[1,1]".into()
    );

    assert_eq!(
        Value::from("[1,2,3,[1,2]]"),
        Value::List(vec!["1".into(), "2".into(), "3".into(), "[1,2]".into()])
    );

    assert_eq!(Value::List(vec![]), "[]".into());

    // dbg!(Value::from("[[[]]]"));

    assert!(Value::from("[[4,4],4,4]") < Value::from("[[4,4],4,4,4]"));
    assert!(Value::from("[1,[2,[3,[4,[5,6,7]]]],8,9]") > Value::from("[1,[2,[3,[4,[5,6,0]]]],8,9]"));

    let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    assert_eq!(part1(input), 13);
}

#[test]
fn real() {
    let input = include_str!("../input/day13.txt");

    assert_eq!(part1(input), 0);
}