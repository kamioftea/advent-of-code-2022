use std::fs;

pub fn run() {
    let contents = fs::read_to_string("res/day-01-input").expect("Failed to read file");
    let sums = parse_input_to_sums(&contents);

    println!(
        "The most calories carried is: {}",
        find_most_calories(&sums)
    );

    println!(
        "The total calories carried by the top three elves is: {}",
        find_top_three_calories(&sums)
    );
}

fn sum_elves(elves: &Vec<Vec<i32>>) -> Vec<i32> {
    elves
        .into_iter()
        .map(|calories| calories.iter().sum())
        .collect()
}

fn find_most_calories(sums: &Vec<i32>) -> i32 {
    *sums.iter()
         .max()
         .unwrap_or(&0)
}

fn find_top_three_calories(sums: &Vec<i32>) -> i32 {
    let (a, b, c) = sums.iter().fold((0, 0, 0), bubble_calorie_sum_into_top_three);

    a + b + c
}

fn bubble_calorie_sum_into_top_three(top_3: (i32, i32, i32), &elf: &i32) -> (i32, i32, i32) {
    match top_3 {
        (a, b, _) if a < elf => (elf, a, b),
        (a, b, _) if b < elf => (a, elf, b),
        (a, b, c) if c < elf => (a, b, elf),
        _ => top_3
    }
}

fn parse_input(input: &String) -> Vec<Vec<i32>> {
    let (output, acc) =
        input.lines()
             .fold(
                 (Vec::new(), Vec::new()),
                 |(output, acc), line|
                     match line.parse::<i32>() {
                         Ok(calories) => (output, [acc, vec!(calories)].concat()),
                         Err(_) => ([output, vec!(acc)].concat(), Vec::new())
                     },
             );

    [output, vec!(acc)].concat()
}

fn parse_input_to_sums(input: &String) -> Vec<i32> {
    let mut output = Vec::new();
    let mut acc = 0;

    input.lines().for_each(
        |line| match line.parse::<i32>() {
            Ok(calories) => acc = acc + calories,
            Err(_) => {
                output.push(acc);
                acc = 0;
            }
        }
    );

    if acc > 0 {
        output.push(acc)
    }

    output
}

#[cfg(test)]
mod tests {
    use crate::day_1::{find_most_calories, find_top_three_calories, parse_input, parse_input_to_sums, sum_elves};

    #[test]
    fn can_parse_sample_input() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();

        let expected = sample_elves();

        assert_eq!(parse_input(&input), expected);
        assert_eq!(parse_input_to_sums(&input), sample_sums());
    }

    fn sample_elves() -> Vec<Vec<i32>> {
        vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]
    }

    fn sample_sums() -> Vec<i32> {
        sum_elves(&sample_elves())
    }

    #[test]
    fn can_sum_elves() {
        assert_eq!(
            sample_sums(),
            vec![6000, 4000, 11000, 24000, 10000]
        )
    }

    #[test]
    fn can_find_most_calories() {
        assert_eq!(find_most_calories(&sample_sums()), 24000)
    }

    #[test]
    fn can_find_top_three_calories() {
        assert_eq!(find_top_three_calories(&sample_sums()), 45000)
    }
}
