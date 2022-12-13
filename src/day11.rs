use core::num;

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: fn(usize) -> usize,
    throw_to: fn(usize) -> usize,
    item_inspections: usize,
}

fn take_turn(monkey_idx: usize, monkies: &mut [Monkey], divide: bool, modu: usize) {
    let monkey = monkies.get_mut(monkey_idx).unwrap();
    let items = std::mem::take(&mut monkey.items);
    let operation = monkey.operation;
    let throw_to = monkey.throw_to;
    monkey.item_inspections += items.len();

    for item in items {
        let item_worry_level = item;

        let item_worry_level = (operation)(item_worry_level);

        let item_worry_level = if divide {
            item_worry_level / 3
        } else {
            item_worry_level % modu
        };

        let next_monkey_index = (throw_to)(item_worry_level);

        monkies
            .get_mut(next_monkey_index)
            .unwrap()
            .items
            .push(item_worry_level);
    }
}

fn round(monkies: &mut [Monkey], divide: bool, modu: usize) {
    for monkey_index in 0..monkies.len() {
        take_turn(monkey_index, monkies, divide, modu);
    }
}

fn rounds(monkies: &mut [Monkey], rounds: usize, modu: usize) -> i64 {
    for _ in 0..rounds {
        for monkey_index in 0..monkies.len() {
            take_turn(monkey_index, monkies, rounds <= 20, modu);
        }
    }

    let mut num_inspections: Vec<_> = monkies.iter().map(|m| m.item_inspections as i64).collect();
    let last_two = num_inspections.len() - 2;
    num_inspections.select_nth_unstable(last_two);
    dbg!(&num_inspections);
    num_inspections[last_two..].iter().product()
}

#[test]
fn test() {
    let mut monkies = [
        Monkey {
            items: vec![79, 98],
            operation: |old| old * 19,
            throw_to: |worry| if worry % 23 == 0 { 2 } else { 3 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |old| old + 6,
            throw_to: |worry| if worry % 19 == 0 { 2 } else { 0 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |old| old * old,
            throw_to: |worry| if worry % 13 == 0 { 1 } else { 3 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![74],
            operation: |old| old + 3,
            throw_to: |worry| if worry % 17 == 0 { 0 } else { 1 },
            item_inspections: 0,
        },
    ];

    //assert_eq!(rounds(&mut monkies, 20, 9999999999), 10605);
    assert_eq!(rounds(&mut monkies, 10000, 96577), 2713310158);
}

#[test]
fn real() {
    let mut monkies = [
        Monkey {
            items: vec![89, 84, 88, 78, 70],
            operation: |old| old * 5,
            throw_to: |old| if old % 7 == 0 { 6 } else { 7 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![76, 62, 61, 54, 69, 60, 85],
            operation: |old| old + 1,
            throw_to: |old| if old % 17 == 0 { 0 } else { 6 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![83, 89, 53],
            operation: |old| old + 8,
            throw_to: |old| if old % 11 == 0 { 5 } else { 3 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![95, 94, 85, 57],
            operation: |old| old + 4,
            throw_to: |old| if old % 13 == 0 { 0 } else { 1 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![82, 98],
            operation: |old| old + 7,
            throw_to: |old| if old % 19 == 0 { 5 } else { 2 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![69],
            operation: |old| old + 2,
            throw_to: |old| if old % 2 == 0 { 1 } else { 3 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![82, 70, 58, 87, 59, 99, 92, 65],
            operation: |old| old * 11,
            throw_to: |old| if old % 5 == 0 { 7 } else { 4 },
            item_inspections: 0,
        },
        Monkey {
            items: vec![91, 53, 96, 98, 68, 82],
            operation: |old| old * old,
            throw_to: |old| if old % 3 == 0 { 4 } else { 2 },
            item_inspections: 0,
        },
    ];
    // assert_eq!(
    //     rounds(&mut monkies, 20, 3 * 5 * 2 * 19 * 13 * 11 * 17 * 7),
    //     55930
    // );
    assert_eq!(
        rounds(&mut monkies, 10000, 3 * 5 * 2 * 19 * 13 * 11 * 17 * 7),
        14636993466
    );
}
