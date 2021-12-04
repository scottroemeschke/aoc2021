use anyhow::{Context, Result};

fn main() -> Result<()> {
    println!("Reading input...");
    let raw_input = common::read_input()?;
    println!("Parsing input...");
    let parsed_input = parse_input(&raw_input);

    println!("Part One:\n");
    let answer = part_one(&parsed_input);
    println!("Q: What is the power consumption of the submarine?");
    println!("A: {:?}", answer);

    println!("\n\nPart Two:\n");
    let answer = part_two(&parsed_input);
    println!("Q: What is the life support rating of the submarine?");
    println!("A: {:?}", answer);

    Ok(())
}

fn parse_input(raw_input: &str) -> Vec<&str> {
    raw_input.lines().map(|s| s.trim()).collect()
}

fn part_one(input: &Vec<&str>) -> u64 {
    let number_length = input.get(0).unwrap().len();
    let (mut ones, mut zeros) = (vec![0; number_length], vec![0; number_length]);
    for s in input {
        for i in 0..number_length {
            let char = s.chars().nth(i).unwrap();
            match char {
                '0' => {
                    zeros[i as usize] = zeros[i as usize] + 1;
                }
                '1' => {
                    ones[i as usize] = ones[i as usize] + 1;
                }
                _ => panic!("unexpected character {}", char),
            }
        }
    }

    let mut gamma = String::with_capacity(number_length);
    let mut epsilon = String::with_capacity(number_length);

    //hacky
    for _ in 0..number_length {
        gamma.push_str("0");
        epsilon.push_str("0");
    }

    for i in 0..number_length {
        let num_ones = ones[i];
        let num_zeros = zeros[i];

        if num_ones > num_zeros {
            gamma.replace_range(i..i + 1, "1");
        }

        if num_ones < num_zeros {
            epsilon.replace_range(i..i + 1, "1");
        }
    }

    let gamma_num = u16::from_str_radix(&gamma, 2).expect("failed to parse gamma number");
    let epsilon_num = u16::from_str_radix(&epsilon, 2).expect("failed to parse epsilon number");

    gamma_num as u64 * epsilon_num as u64
}

fn get_bit_at(input: &u16, n: u8) -> Result<bool, ()> {
    if n < 16 {
        Ok(input & (1 << n) != 0)
    } else {
        Err(())
    }
}

fn part_two(input: &Vec<&str>) -> u64 {
    let oxygen_generator_rating_as_string = recurse(None, input, BitCriteria::OxygenGenerator);
    let co2_scrubber_rating_as_string = recurse(None, input, BitCriteria::Co2Scrubber);
    let ogr = u64::from_str_radix(oxygen_generator_rating_as_string, 2)
        .expect("couldn't parse oxygen generator rating string into decimal");
    let csr = u64::from_str_radix(co2_scrubber_rating_as_string, 2)
        .expect("couldn't parse co2 scrubber rating into decimal");
    ogr * csr
}

enum BitCriteria {
    OxygenGenerator,
    Co2Scrubber,
}

fn recurse<'input>(
    position: Option<usize>,
    input: &Vec<&'input str>,
    bit_criteria: BitCriteria,
) -> &'input str {
    let mut position = position.unwrap_or(0);
    let (mut num_ones, mut num_zeros) = (0, 0);
    for s in input {
        let char_at_pos = s.chars().nth(position).unwrap();
        match char_at_pos {
            '0' => {
                num_zeros += 1;
            }
            '1' => {
                num_ones += 1;
            }
            _ => panic!("unexpected character!"),
        }
    }

    let required_character_at_position = match bit_criteria {
        BitCriteria::OxygenGenerator => {
            if num_ones >= num_zeros {
                '1'
            } else {
                '0'
            }
        }
        BitCriteria::Co2Scrubber => {
            if num_zeros <= num_ones {
                '0'
            } else {
                '1'
            }
        }
    };

    let input_filtered_for_required_character_at_position = input
        .iter()
        .filter(|s| {
            s.chars()
                .nth(position)
                .expect("missing character at expected position")
                == required_character_at_position
        })
        .map(|s| *s)
        .collect::<Vec<&str>>();

    match input_filtered_for_required_character_at_position.len() {
        1 => {
            return input_filtered_for_required_character_at_position
                .get(0)
                .expect("this should never happen");
        }
        0 => {
            panic!("we filtered down to zero matching lines, something is wrong");
        }
        _ => recurse(
            Some(position + 1),
            &input_filtered_for_required_character_at_position,
            bit_criteria,
        ),
    }
}

#[cfg(test)]
mod test {
    use crate::{part_one, part_two};

    #[test]
    fn test_part_one() {
        //using example from prompt
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let result = part_one(&input);
        assert_eq!(result, 198);
    }

    #[test]
    fn test_part_two() {
        let input = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let result = part_two(&input);
        assert_eq!(result, 230);
    }
}
