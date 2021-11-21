extern crate rand;
use rand::thread_rng;
use rand::seq::SliceRandom;

static NUMBERS_RANGE: std::ops::Range<i32> = 1..10;
static MATRIX_RANGE: std::ops::Range<usize> = 0..9;
static QUADRANT_RANGES: [std::ops::Range<usize>;3]= [0..3, 3..6, 6..9];

struct Sudoku {
    board: [[i32;9];9]
}

impl Sudoku {
    fn new() -> Self {
        let mut sudoku = Self { board: [[0;9];9] };

        sudoku.fill_numbers();
        sudoku
    }

    fn fill_numbers(&mut self) {
        let mut rng = thread_rng();
        let mut vector: Vec<i32> = NUMBERS_RANGE.clone().collect();
        vector.shuffle(&mut rng);

        for row_index in MATRIX_RANGE.clone() {
            if row_index == 0 {
                for column in MATRIX_RANGE.clone() {
                    self.board[row_index][column] = vector[column];
                }
            }
            else {
                loop {
                    vector.shuffle(&mut rng);
                    for column in MATRIX_RANGE.clone() {
                        self.board[row_index][column] = vector[column];
                    }

                    let columns_verified = Self::verify_columns(self.board);
                    let quadrants_verified = Self::verify_quadrants(self.board);

                    if columns_verified && quadrants_verified {
                        break;
                    }
                }
            }
        }
    }

    fn verify_columns(board: [[i32;9];9]) -> bool {
        let mut column_elements: Vec<i32>;

        let mut result = true;
        for column_index in MATRIX_RANGE.clone() {
            column_elements = Vec::new();
            for row_index in MATRIX_RANGE.clone() {
                column_elements.push(board[row_index][column_index]);
            }

            result = result && !Self::has_duplicates(column_elements.clone());
        }

        result
    }

    fn verify_quadrants(board: [[i32;9];9]) -> bool {
        let mut valid = true;

        for row_iter in QUADRANT_RANGES.clone() {
            for column_iter in QUADRANT_RANGES.clone() {
                let mut elements: Vec<i32> = Vec::new();
                for row_index in row_iter.clone() {
                    for column_index in column_iter.clone() {
                        elements.push(board[row_index][column_index]);
                    }
                }

                valid = valid && !Self::has_duplicates(elements.clone());
            }
        }

        valid
    }

    fn has_duplicates(mut elements: Vec<i32>) -> bool {
        elements.sort();

        let mut previous_element = elements[0];
        for index in 1..elements.len() {
            let current_element = elements[index];
            if previous_element == 0 || current_element == 0 {
                previous_element = current_element;
                continue;
            }

            if previous_element == current_element {
                return true
            }

            previous_element = current_element;
        }

        false
    }
}

fn main() {
    let sudoku = Sudoku::new();
    println!("Creating sudoku");
    for row in sudoku.board {
        println!("{:?}", row);
    }
}

#[test]
fn sudoku_board_is_correct() {
    let sudoku = Sudoku::new();
    for index in 0..9 {
        println!("{:?}", sudoku.board[index]);
    }

    for index in 0..9 {
        let mut row = sudoku.board[index].clone();
        row.sort();

        assert_eq!(row, [1, 2, 3, 4, 5, 6, 7, 8, 9]);

        let mut vector: Vec<i32> = Vec::new();
        for row_index in 0..9 {
            vector.push(sudoku.board[row_index][index]);
        }

        vector.sort();
        assert_eq!(vector, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
