---
day: 10
tags: post
header: 'Day 10: Cathode-Ray Tube'
---
 
Today I need to interpret the visual instructions of my broken comms device since the screen is now waterlogged.

## Instructions to signals

The raw output is a series of instructions that update a single register. First I'll turn the puzzle input into a list 
of enum values. I'll create the enum, parse each line to an `Instruction`, and collect the lines into a `Vec`, and 
use the example as a test case.

```rust
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Instruction {
    ADDX(isize),
    NOOP,
}
// ...
fn parse_input(input: &String) -> Vec<Instruction> {
    input.lines().map(parse_instruction).collect()
}

fn parse_instruction(line: &str) -> Instruction {
    if line.starts_with("addx") {
        let (_, value) = line.split_once(" ").unwrap();
        ADDX(value.parse::<isize>().unwrap())
    } else {
        NOOP
    }
}
//...
#[test]
fn can_parse() {
    let input = "noop
addx 3
addx -5".to_string();

    assert_eq!(parse_input(&input), vec![NOOP, ADDX(3), ADDX(-5)])
}
```

The values of interest are the signals produced by reading the register once a cycle, and notablly `ADDX` takes two 
cycles before the update is appied. I can represent this by pushing the current register value the relevant number 
of times for the instruction being parsed. This gives the list of signals needed for the puzzle answers. Again there 
is a test example here, I add an additional `NOOP` so that the value after the second `ADDX` is emitted once.

```rust
fn to_signals(instructions: &Vec<Instruction>) -> Vec<isize> {
    let mut register = 1;
    let mut signals = Vec::new();
    for &instruction in instructions {
        match instruction {
            ADDX(x) => {
                signals.push(register);
                signals.push(register);
                register = register + x;
            }
            NOOP => signals.push(register)
        }
    }

    signals
}
// ...
#[test]
fn can_generate_signals() {
    assert_eq!(
        to_signals(&vec![NOOP, ADDX(3), ADDX(-5), NOOP]),
        vec!(1, 1, 1, 4, 4, -1)
    )
}
```

## Sampling the signal

Part one requires taking the value after 20 cycles, then every 40 cycles after that. That done, sum each of those 
values. Since I have a list of the signals, I can chain `Itertools` extensions to achieve this, test it, and then 
apply the puzzle input.

```rust
fn sample_and_sum_signal_strength(instructions: &Vec<Instruction>) -> isize {
    to_signals(instructions)
        .iter()
        .enumerate()
        .dropping(19)
        .step_by(40)
        .take(6)
        .map(|(step, &signal)| isize::try_from(step + 1).unwrap() * signal)
        .sum()
}
// ... 
#[test]
fn can_sum_signal_samples() {
    assert_eq!(
        sample_and_sum_signal_strength(&sample_instructions()),
        13140
    )
}
// Note: sample_instructions() is created from the rather long example in the puzzle, 
// that I will not repeat here# as it is quite tedious to scroll past.
// ...
pub fn run() {
    let contents =
        fs::read_to_string("res/day-10-input").expect("Failed to read file");

    let instructions = parse_input(&contents);

    println!(
        "The sum of sampled signal strengths is: {}",
        sample_and_sum_signal_strength(&instructions)
    );
}
// The sum of sampled signal strengths is: 14240
```

## Signal to screen

That done, I find out that the register holds the `x` position of the centre of a three pixel "sprite" the height of 
the screen. It is intended that the screen cycles through each of it's 40 x 6 pixels, and if the sprite overlaps the 
current pixel, it should be displayed. The task is to simulate this and determine which letters would be displayed 
on the screen if it were working.

I can determine the current `x` by taking the modulus of the index of the current signal. Add either a █ or . 
depending on if the index is within one of the signal, and therefore withing the "sprite". Finally, if this is the 
last pixel in a row, add a new line. This also explains why the example for part one was so long, I need 240 pixels 
to render the full example.

```rust
fn draw_pixels(instructions: &Vec<Instruction>) -> String {
    let mut lines = String::new();

    for (i, &signal) in to_signals(instructions).iter().enumerate() {
        let pos = isize::try_from(i % 40).unwrap();

        lines.push(
            if pos.abs_diff(signal) <= 1 { '█' } else { '.' }
        );

        if pos == 39 {
            lines.push('\n')
        }
    }

    lines
}
// ...
#[test]
fn can_draw_pixels() {
    let expected = "██..██..██..██..██..██..██..██..██..██..
███...███...███...███...███...███...███.
████....████....████....████....████....
█████.....█████.....█████.....█████.....
██████......██████......██████......████
███████.......███████.......███████.....\n".to_string();

    assert_eq!(draw_pixels(&sample_instructions()), expected);
}
// ...
pub fn run() {
    let contents =
        fs::read_to_string("res/day-10-input").expect("Failed to read file");

    let instructions = parse_input(&contents);

    println!(
        "The sum of sampled signal strengths is: {}",
        sample_and_sum_signal_strength(&instructions)
    );

    println!(
        "The screen shows: \n{}",
        draw_pixels(&instructions)
    );
}
// The sum of sampled signal strengths is: 14240
// The screen shows:
// ███..█....█..█.█....█..█.███..████.█..█.
// █..█.█....█..█.█....█.█..█..█....█.█..█.
// █..█.█....█..█.█....██...███....█..████.
// ███..█....█..█.█....█.█..█..█..█...█..█.
// █....█....█..█.█....█.█..█..█.█....█..█.
// █....████..██..████.█..█.███..████.█..█.
// 
// -- took 1.92ms
```
