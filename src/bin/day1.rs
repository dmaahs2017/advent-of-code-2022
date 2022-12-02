fn main() {
    let input = include_str!("../../inputs/day1/input.txt");
    println!("The elf with the most calories carries {} calories", solve_puzzle_1(input));
    println!("The sum of 3 elves with the most calories is {}", solve_puzzle_2(input));
}

/// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
fn solve_puzzle_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .expect("input was empty")
}

/// Find the top three Elves carrying the most Calories. How many Calories are those Elves carrying in total?
fn solve_puzzle_2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split_whitespace()
                .map(|calories| calories.parse::<usize>().unwrap())
                .sum()
        }).fold([0, 0, 0], |mut acc, elf_bag: usize| {
            let min = acc.iter_mut().min().unwrap();
            *min = elf_bag.max(*min);
            acc
        }).iter().sum()
}

#[test]
fn solve_puzzle_2_works() {
    let input = include_str!("../../inputs/day1/sample.txt");
    assert_eq!(solve_puzzle_2(input), 45000)
}

#[test]
fn solve_puzzle_1_works() {
    let input = include_str!("../../inputs/day1/sample.txt");
    assert_eq!(solve_puzzle_1(input), 24000)
}


