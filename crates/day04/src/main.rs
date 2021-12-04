use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Game {
    called_numbers: Vec<i32>,
    boards: Vec<Board>
}

impl Game {
    pub fn call_number(&mut self, n: &i32) {
        for i in 0..self.boards.len() {
            self.boards[i].call_number(n);
        }
    }

    pub fn has_winner(&self) -> bool {
        for b in &self.boards {
            if b.is_winner() {
                return true;
            }
        }
        false
    }
}

impl From<&Vec<String>> for Game {
    fn from(lines: &Vec<String>) -> Self {
        let mut boards = vec![];
        let called_numbers = lines[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let board_count = (lines.len() - 1) / 6;
        for i in 0..board_count {
            let board_start = (i * 6) + 1;
            let board_lines = (board_start + 1)..(board_start + 6);
            let mut board_grid = vec![];
            for x in board_lines {
                let numbers = lines[x].split(" ").filter_map(|e| e.parse::<i32>().ok()).collect::<Vec<_>>();
                board_grid.push(
                    numbers
                );
            }
            boards.push(
                board_grid.into()
            )
        }
        Game {
            called_numbers,
            boards,
        }
    }
}

impl From<Vec<Vec<i32>>> for Board {
    fn from(grid: Vec<Vec<i32>>) -> Self {
        Board {
            grid,
            marked: vec![],
        }
    }
}

struct Board {
    grid: Vec<Vec<i32>>,
    marked: Vec<(i32, (usize, usize))>
}

impl Board {
    pub fn call_number(&mut self, n: &i32) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                let cell = self.grid[row][col];
                if cell == *n {
                    self.marked.push((cell, (row, col)));
                }
            }
        }
    }

    pub fn is_winner(&self) -> bool {
        let mut winning_rows: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut winning_col: HashMap<usize, Vec<usize>> = HashMap::new();
        for (_n, (row, col)) in &self.marked {
            let row_record = winning_rows.entry(*row).or_insert(Vec::new());
            row_record.push(*col);
            let col_record = winning_col.entry(*col).or_insert(Vec::new());
            col_record.push(*row);
        }
        for (_row, marked_col) in winning_rows {
            if marked_col.len() == 5 {
                return true;
            }
        }

        for (_col, marked_row) in winning_col {
            if marked_row.len() == 5 {
                return true;
            }
        }
        false
    }

    pub fn get_uncalled_numbers(&self) -> Vec<i32> {
        let mut o = vec![];
        let called = self.marked.iter().map(|(n, _)| *n).collect::<Vec<_>>();
        for row in &self.grid {
            for n in row {
                if called.contains(&n) {
                    continue;
                } else {
                    o.push(*n);
                }
            }
        }
        o
    }
}

fn pt1(input: &Vec<String>) -> i32 {
    let mut game: Game = input.into();
    for i in 0..game.called_numbers.len() {
        let n = game.called_numbers[i];
        game.call_number(&n);
        if game.has_winner() {
            let winners: Vec<&Board> = game.boards.iter().filter(|b| b.is_winner()).collect();
            let first_winner = winners[0];
            let uncalled_sum = first_winner.get_uncalled_numbers().iter().fold(0, |acc, n| acc + n);
            return uncalled_sum * n;
        }
    }
    // lol?
    0

}

pub fn main() -> std::io::Result<()> {
    let f = File::open("data/4")?;
    let reader: BufReader<File> = BufReader::new(f);
    let input: Vec<String> = reader.lines().map(|i| i.unwrap()).collect();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    static TEST: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
 2  0 12  3  7";

    #[test]
    fn test_parse() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let game: Game = input.into();
        assert_eq!(game.called_numbers[0], 7);
        assert_eq!(game.boards[0].grid[0][0], 22);
        assert_eq!(game.boards[0].grid[0][4], 0);
        assert_eq!(game.boards[2].grid[0][0], 14);
    }

    #[test]
    fn test_call() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let mut game: Game = input.into();
        let board = &mut game.boards[0];
        board.call_number(&24);
        assert_eq!(board.marked.len(), 1);
    }

    #[test]
    fn test_win_first_row() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let mut game: Game = input.into();
        let board = &mut game.boards[0];
        let calls = vec![
            22,
            13,
            //17,
            11,
            0,
        ];
        for n in calls {
            board.call_number(&n);
        }
        assert_eq!(board.is_winner(), false);
        board.call_number(&17);
        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn test_win_first_col() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let mut game: Game = input.into();
        let board = &mut game.boards[0];
        let calls = vec![
            22,
            8,
            21,
            6,
            1,
        ];
        for n in calls {
            board.call_number(&n);
        }
        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn test_win_last_col() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let mut game: Game = input.into();
        let board = &mut game.boards[0];
        let calls = vec![
            0,
            24,
            7,
            5,
            19,
        ];
        for n in calls {
            board.call_number(&n);
        }
        assert_eq!(board.is_winner(), true);
    }

    #[test]
    fn test_example() {
        let input: &Vec<String> = &TEST.lines().map(|i| i.to_string()).collect();
        let answer = pt1(input);
        assert_eq!(answer, 4512);
    }

}
