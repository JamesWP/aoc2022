use std::{str::Chars, iter::Peekable};

#[derive(Debug, PartialEq)]
enum Value {
    Literal(i32),
    List(Vec<Value>),
}

impl<'a> From<&mut Peekable<Chars<'a>>> for  Value {
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
                        _ => todo!()
                    };
                }

                Value::List(values)
            },
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
                    break
                };
                let number = String::from_iter(number.into_iter());
                Value::Literal(number.parse().unwrap())
            }
            _ => todo!()
        }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        let mut input = value.chars().peekable();
        Value::from(&mut input)
    }
}

#[test]
fn test() {
    assert_eq!(Value::Literal(1), "1".into());
    assert_eq!(Value::List(vec![Value::Literal(1)]), "[1]".into());
    assert_eq!(Value::List(vec![Value::Literal(1),Value::Literal(1)]), "[1,1]".into());

    assert_eq!(Value::from("[1,2,3,[1,2]]"), Value::List(vec!["1".into(), "2".into(), "3".into(), "[1,2]".into()]));

    assert_eq!(Value::List(vec![]), "[]".into());

    dbg!(Value::from("[[[]]]"));
}