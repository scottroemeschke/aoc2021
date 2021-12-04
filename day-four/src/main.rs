use anyhow::Result;
use ndarray::Array2;

fn main() -> Result<()> {
    println!("Reading input...");
    let raw_input = common::read_input()?;
    println!("Parsing input...");
    let parsed_input = parse_input(&raw_input)?;

    println!("Part One:\n");
    let answer = part_one(parsed_input.clone());
    println!("Q: What will your final score be if you choose the first winning board?");
    println!("A: {:?}", answer);

    println!("\n\nPart Two:\n");
    let answer = part_two(parsed_input.clone());
    println!("Q: Once it wins, what would its final score be?");
    println!("A: {:?}", answer);

    Ok(())
}

#[derive(Debug, PartialEq, Clone, Default)]
struct BingoSlot {
    number: u8,
    marked: bool,
}

impl BingoSlot {
    fn new_unmarked_with_number(number: u8) -> Self {
        BingoSlot {
            number,
            marked: false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct InputNumbersAndBoards {
    numbers: Vec<u8>,
    boards: Vec<Array2<BingoSlot>>,
}

fn parse_input(raw_input: &str) -> Result<InputNumbersAndBoards> {
    let mut input_numbers = Vec::new();
    let mut num_vertical_lines_on_current_board = 0;
    let mut boards = Vec::new();
    let mut working_board = Array2::default((5, 5));
    for (index, mut line) in raw_input.lines().enumerate() {
        line = line.trim();
        match index {
            0 => {
                //parse the input numbers
                input_numbers = line
                    .split(",")
                    .map(|s| {
                        s.parse::<u8>()
                            .expect("failed to parse an input number to a u8")
                    })
                    .collect()
            }
            1 => continue, //empty line before the boards start
            _ => {
                if line.is_empty() {
                    if num_vertical_lines_on_current_board != 5 {
                        panic!("we hit an empty line, past index 1, but we haven't read four vertical lines or \
                        board numbers yet.. this should never happen, check your input")
                    } else {
                        //reset it and move to the next line if there is one where we expect a new board
                        num_vertical_lines_on_current_board = 0;
                        continue;
                    }
                }

                //we know we are reading a board
                for (i, n) in line
                    .split_whitespace()
                    .map(|s| {
                        s.parse::<u8>()
                            .expect("failed to parse number in board to u8")
                    })
                    .enumerate()
                {
                    //fill in all the numbers for the row we are now
                    working_board.row_mut(num_vertical_lines_on_current_board)[i] =
                        BingoSlot::new_unmarked_with_number(n);
                }

                //now that we are done reading that row, mark the counter up
                num_vertical_lines_on_current_board += 1;

                //if we have hit 5, copy the working board into our vec of all the boards
                if num_vertical_lines_on_current_board == 5 {
                    boards.push(working_board.clone())
                }
            }
        }
    }
    Ok(InputNumbersAndBoards {
        numbers: input_numbers,
        boards,
    })
}

fn play_game(mut input: InputNumbersAndBoards) -> Vec<(usize, Array2<BingoSlot>, u8)> {
    let mut winning_boards_with_final_number_and_index =
        Vec::<(usize, Array2<BingoSlot>, u8)>::new();

    for current_bingo_number in &input.numbers {
        for board in &mut input.boards {
            for slot in board.iter_mut() {
                if slot.number == *current_bingo_number {
                    slot.marked = true;
                }
            }
        }

        for (board_index, board) in input.boards.iter().enumerate() {
            if is_win(board) {
                //check if it's already in our winners list
                if let None = winning_boards_with_final_number_and_index
                    .iter()
                    .find(|(i, b, number)| *i == board_index)
                {
                    winning_boards_with_final_number_and_index.push((
                        board_index,
                        board.clone(),
                        *current_bingo_number,
                    ))
                }
            }
        }
    }

    winning_boards_with_final_number_and_index
}

fn part_one(mut input: InputNumbersAndBoards) -> u64 {
    let winning_boards = play_game(input);
    let (_, board, winning_number) = winning_boards.first().expect("no winning boards");
    return calculate_winning_board_score(board, *winning_number);
}

fn part_two(mut input: InputNumbersAndBoards) -> u64 {
    let winning_boards = play_game(input);
    let (_, board, winning_number) = winning_boards.last().expect("no winning boards");
    return calculate_winning_board_score(board, *winning_number);
}

fn is_win(board: &Array2<BingoSlot>) -> bool {
    let mut num_marked = 0;
    let mut winning = false;
    for row in board.rows() {
        for slot in row {
            if slot.marked == true {
                num_marked += 1;
            }
        }
        if num_marked == 5 {
            //bingo!
            winning = true;
            break;
        } else {
            //reset counter
            num_marked = 0;
        }
    }
    for column in board.columns() {
        for slot in column {
            if slot.marked == true {
                num_marked += 1;
            }
        }
        if num_marked == 5 {
            //bingo!
            winning = true;
            break;
        } else {
            //reset counter
            num_marked = 0;
        }
    }
    winning
}

fn calculate_winning_board_score(board: &Array2<BingoSlot>, winning_number: u8) -> u64 {
    let summed = board.into_iter().fold(0u64, |sum, slot| {
        if !slot.marked {
            sum + slot.number as u64
        } else {
            sum
        }
    });
    summed as u64 * winning_number as u64
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part_one, part_two, BingoSlot, InputNumbersAndBoards};
    use ndarray::array;

    fn test_input() -> InputNumbersAndBoards {
        let board_1 = array![
            [22u8, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19]
        ]
        .mapv(|n| BingoSlot::new_unmarked_with_number(n));

        let board_2 = array![
            [3u8, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ]
        .mapv(|n| BingoSlot::new_unmarked_with_number(n));

        let board_3 = array![
            [14u8, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ]
        .mapv(|n| BingoSlot::new_unmarked_with_number(n));

        InputNumbersAndBoards {
            numbers: vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ],
            boards: vec![board_1, board_2, board_3],
        }
    }

    #[test]
    fn test_parsing() {
        let example_input = r#"
        
        7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19
        
         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6
        
        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
        
        "#
        .trim();

        let parsed = parse_input(&example_input)
            .expect("failed to parse input numbers and boards in test example");

        let expected = test_input();

        assert_eq!(expected, parsed)
    }

    #[test]
    fn test_part_one() {
        let input = test_input();
        let answer = part_one(input);
        assert_eq!(answer, 4512);
    }

    #[test]
    fn test_part_two() {
        let input = test_input();
        let answer = part_two(input);
        assert_eq!(answer, 1924);
    }
}
