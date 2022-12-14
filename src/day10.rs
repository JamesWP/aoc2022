struct MachineState {
    reg_x: i32,
    cycle: i32,
}

impl Default for MachineState {
    fn default() -> Self {
        Self { reg_x: 1, cycle: 1 }
    }
}

trait Visit {
    fn during_cycle(&mut self, cycle_number: i32, state: &MachineState);
}

struct CycleSnooper {
    total_signal_strength: i32,
}

impl Default for CycleSnooper {
    fn default() -> Self {
        Self {
            total_signal_strength: 0,
        }
    }
}

impl Visit for CycleSnooper {
    fn during_cycle(&mut self, cycle_number: i32, state: &MachineState) {
        if 0 != ((cycle_number - 20) % 40) {
            return;
        }

        let signal_strength = cycle_number * state.reg_x;

        self.total_signal_strength += signal_strength;
    }
}

struct CRTSnooper {}

impl Default for CRTSnooper {
    fn default() -> Self {
        Self {}
    }
}

impl Visit for CRTSnooper {
    fn during_cycle(&mut self, cycle_number: i32, state: &MachineState) {
        // dbg!(cycle_number, (cycle_number-1), state.reg_x, state.reg_x - (cycle_number-1));
        if (state.reg_x - ((cycle_number%40)-1)).abs() < 2 {
            print!("#");
            // dbg!("#");
        } else {
            print!(" ");
            // dbg!(".");
        }
        if (cycle_number) % 40 == 0 {
            // dbg!(cycle_number);
            println!();
        }
    }
}

fn machine<V: Visit>(input: &str, snoop: &mut V) {
    let mut state = MachineState::default();

    for line in input.lines() {
        if line.starts_with("addx") {
            let value: i32 = line.split_once(" ").unwrap().1.parse().unwrap();

            snoop.during_cycle(state.cycle, &state);
            state.cycle += 1;
            snoop.during_cycle(state.cycle, &state);
            state.cycle += 1;
            state.reg_x += value;
        } else if line.starts_with("noop") {
            snoop.during_cycle(state.cycle, &state);
            state.cycle += 1;
        } else {
            todo!()
        }
    }
}

fn part1(input: &str) -> i32 {
    let mut snoop = CycleSnooper::default();

    machine(input, &mut snoop);

    snoop.total_signal_strength
}

fn part2(input: &str) {
    let mut snoop = CRTSnooper::default();

    machine(input, &mut snoop);
}

#[test]
fn test() {
    let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(part1(input), 13140);
    part2(input);
}

#[test]
fn real() {
    let input = include_str!("../input/day10.txt");

    assert_eq!(part1(input), 13680);
    part2(input);
}
