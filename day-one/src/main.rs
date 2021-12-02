use anyhow::{Context, Result};
use common;

fn main() -> Result<()> {
    println!("Reading input...");
    let raw_input = common::read_input()?;
    println!("Parsing input...");
    let parsed_input = parse_input(&raw_input);

    println!("Part One:\n");
    let answer = part_one(&parsed_input);
    println!("Q: How many measurements are larger than the previous measurement?");
    println!("A: {:?}", answer);

    println!("\n\nPart Two:\n");
    let answer = part_two(&parsed_input);
    println!("Q: How many sums are larger than the previous sum?");
    println!("A: {:?}", answer);

    Ok(())
}

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .into_iter()
        .map(|s| s.trim())
        .map(|s| {
            s.parse::<i32>()
                .with_context(|| format!("failed to parse input string: {:?}, into an i32", s))
                .unwrap() //panic on the first string that fails to parse
        })
        .collect::<Vec<i32>>()
}

fn part_one(numbers: &[i32]) -> i32 {
    let mut count_greater_than_previous = 0;
    let mut last_num = None;

    for number in numbers {
        //skip the first item
        if let Some(last_number_value) = last_num {
            if number > last_number_value {
                count_greater_than_previous += 1;
            }
        }
        last_num = Some(number);
    }

    count_greater_than_previous
}

fn part_two(numbers: &[i32]) -> i32 {
    let mut sum_count_greater_than_previous = 0;
    let mut last_sum = None;

    for nums in numbers.windows(3) {
        let new_sum = nums[0] + nums[1] + nums[2];
        if let Some(last_sum_value) = last_sum {
            if new_sum > last_sum_value {
                sum_count_greater_than_previous += 1;
            }
        }
        last_sum = Some(new_sum)
    }

    sum_count_greater_than_previous
}

#[cfg(test)]
mod tests {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        let nums = [1, 2, 5, 5, 2];
        let answer = part_one(&nums);
        assert_eq!(answer, 2); //calc by hand
    }

    #[test]
    fn test_part_two() {
        let nums = [85, 120, 194, 21, 15, 250, 354, 908, 342];
        let answer = part_two(&nums);
        assert_eq!(answer, 4); //calc by hand
    }
}
