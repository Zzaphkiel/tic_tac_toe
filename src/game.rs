use rand::Rng;
use std::cmp;

pub const BOARD_WIDTH: usize = 3;
pub const CHESS_NUM_FOR_WIN: usize = 3;

#[derive(Debug)]
pub enum Player {
    Max,
    Min,
}

#[derive(Debug)]
pub enum GameStage {
    Ongoing,
    MaxWin,
    MinWin,
    Draw,
}

impl Player {
    pub fn to_chess(&self) -> Chess {
        match self {
            Player::Max => Chess::Cross,
            Player::Min => Chess::Circle,
        }
    }

    pub fn exchange(&self) -> Player {
        match self {
            Player::Max => Player::Min,
            Player::Min => Player::Max,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Chess {
    Cross,
    Circle,
}

impl Chess {
    fn to_result(&self) -> GameStage {
        match self {
            Chess::Cross => GameStage::MaxWin,
            Chess::Circle => GameStage::MinWin,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board(pub [[Option<Chess>; BOARD_WIDTH]; BOARD_WIDTH]);

impl Board {
    pub fn new() -> Board {
        Board([[None; BOARD_WIDTH]; BOARD_WIDTH])
    }

    pub fn filled(&self, position: (usize, usize)) -> bool {
        !self.0[position.0][position.1].is_none()
    }

    pub fn put(&self, chess: Chess, position: (usize, usize)) -> Board {
        let mut board = self.clone();
        board.0[position.0][position.1] = Some(chess);

        board
    }

    pub fn terminal_test(&self) -> GameStage {
        let check_vertical = |row: usize, col: usize| -> bool {
            if row + CHESS_NUM_FOR_WIN > BOARD_WIDTH {
                return false;
            }

            for i in 1..CHESS_NUM_FOR_WIN {
                if self.0[row + i - 1][col] != self.0[row + i][col] {
                    return false;
                }
            }

            true
        };

        let check_horizontal = |row: usize, col: usize| -> bool {
            if col + CHESS_NUM_FOR_WIN > BOARD_WIDTH {
                return false;
            }

            for i in 1..CHESS_NUM_FOR_WIN {
                if self.0[row][col + i - 1] != self.0[row][col + i] {
                    return false;
                }
            }

            true
        };

        let check_down_diagonal = |row: usize, col: usize| -> bool {
            if row + CHESS_NUM_FOR_WIN > BOARD_WIDTH {
                return false;
            }

            if col + CHESS_NUM_FOR_WIN > BOARD_WIDTH {
                return false;
            }

            for i in 1..CHESS_NUM_FOR_WIN {
                if self.0[row + i - 1][col + i - 1] != self.0[row + i][col + i] {
                    return false;
                }
            }

            true
        };

        let check_up_diagonal = |row: usize, col: usize| -> bool {
            if col + CHESS_NUM_FOR_WIN > BOARD_WIDTH {
                return false;
            }

            if row < CHESS_NUM_FOR_WIN - 1 {
                return false;
            }

            for i in 1..CHESS_NUM_FOR_WIN {
                if self.0[row - i + 1][col + i - 1] != self.0[row - i][col + i] {
                    return false;
                }
            }

            true
        };

        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                if check_vertical(row, col)
                    || check_horizontal(row, col)
                    || check_up_diagonal(row, col)
                    || check_down_diagonal(row, col)
                {
                    if let Some(chess) = self.0[row][col] {
                        return chess.to_result();
                    }
                }
            }
        }

        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                if !self.filled((row, col)) {
                    return GameStage::Ongoing;
                }
            }
        }

        GameStage::Draw
    }

    //                           (x,     y)
    pub fn actions(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];

        for row in 0..BOARD_WIDTH {
            for col in 0..BOARD_WIDTH {
                if !self.filled((row, col)) {
                    result.push((row, col));
                }
            }
        }

        result
    }

    //                                (x,     y)
    pub fn minimax_decision(&self) -> (usize, usize) {
        let mut max_value = i32::min_value();

        let mut win_positions = vec![];
        let mut draw_positions = vec![];
        let mut lose_positions = vec![];

        for action in self.actions() {
            let value = self.put(Chess::Cross, action).min_value();
            if value > max_value {
                max_value = value;
            }

            // println!(
            //     "position = {:?}, value = {}",
            //     (action.0 + 1, action.1 + 1),
            //     value
            // );

            match value {
                1 => win_positions.push(action),
                0 => draw_positions.push(action),
                _ => lose_positions.push(action),
            }
        }

        fn rand_usize(min: usize, max: usize) -> usize {
            rand::thread_rng().gen_range(min..max)
        }

        match max_value {
            1 => win_positions[rand_usize(0, win_positions.len())],
            0 => draw_positions[rand_usize(0, draw_positions.len())],
            _ => lose_positions[rand_usize(0, lose_positions.len())],
        }
    }

    pub fn max_value(&self) -> i32 {
        let mut value = i32::min_value();

        match self.terminal_test() {
            GameStage::MaxWin => value = 1,
            GameStage::MinWin => value = -1,
            GameStage::Draw => value = 0,
            GameStage::Ongoing => {
                for action in self.actions() {
                    let new_board = self.put(Chess::Cross, action);
                    value = cmp::max(value, new_board.min_value());
                }
            }
        }

        value
    }

    pub fn min_value(&self) -> i32 {
        let mut value = i32::max_value();

        match self.terminal_test() {
            GameStage::MaxWin => value = 1,
            GameStage::MinWin => value = -1,
            GameStage::Draw => value = 0,
            GameStage::Ongoing => {
                for action in self.actions() {
                    let new_board = self.put(Chess::Circle, action);
                    value = cmp::min(value, new_board.max_value());
                }
            }
        }

        value
    }

    pub fn print(&self) {
        print!("    ");
        for i in 0..BOARD_WIDTH {
            print!("{}   ", i + 1);
        }
        println!("");

        for i in 0..BOARD_WIDTH {
            print!(" {} ", i + 1);
            for j in 0..BOARD_WIDTH {
                let mut ch: char = ' ';
                if let Some(chess) = self.0[i][j] {
                    ch = match chess {
                        Chess::Circle => 'O',
                        Chess::Cross => 'X',
                    }
                }
                print!(" {:1} ", ch);
                if j != BOARD_WIDTH - 1 {
                    print!("│");
                }
            }

            println!("");
            if i != BOARD_WIDTH - 1 {
                print!("   ───");
                for _ in 0..BOARD_WIDTH - 1 {
                    print!("┼───");
                }
                println!("");
            }
        }

        println!("");
    }
}
