use rand::Rng;
use std::cmp;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Player {
    Max,
    Min,
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
    fn to_player(&self) -> Player {
        match self {
            Chess::Cross => Player::Max,
            Chess::Circle => Player::Min,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Board(pub [[Option<Chess>; 3]; 3]);

impl Board {
    pub fn new() -> Board {
        Board([[None; 3]; 3])
    }

    pub fn filled(&self, position: (usize, usize)) -> bool {
        !self.0[position.0][position.1].is_none()
    }

    pub fn put(&self, chess: Chess, position: (usize, usize)) -> Board {
        let mut board = self.clone();
        board.0[position.0][position.1] = Some(chess);

        board
    }

    pub fn draw_test(&self) -> bool {
        for col in 0..3 {
            for row in 0..3 {
                if !self.filled((row, col)) {
                    return false;
                }
            }
        }

        true
    }

    pub fn terminal_test(&self) -> Option<Player> {
        for row in 0..3 {
            if self.0[row][0] == self.0[row][1] && self.0[row][1] == self.0[row][2] {
                if let Some(chess) = self.0[row][0] {
                    return Some(chess.to_player());
                }
            }
        }

        for col in 0..3 {
            if self.0[0][col] == self.0[1][col] && self.0[1][col] == self.0[2][col] {
                if let Some(chess) = self.0[0][col] {
                    return Some(chess.to_player());
                }
            }
        }

        if self.0[0][0] == self.0[1][1] && self.0[1][1] == self.0[2][2] {
            if let Some(chess) = self.0[1][1] {
                return Some(chess.to_player());
            }
        }

        if self.0[2][0] == self.0[1][1] && self.0[1][1] == self.0[0][2] {
            if let Some(chess) = self.0[1][1] {
                return Some(chess.to_player());
            }
        }

        None
    }

    //                           (x,     y)
    pub fn actions(&self) -> Vec<(usize, usize)> {
        let mut result = vec![];

        for row in 0..3 {
            for col in 0..3 {
                if self.0[row][col].is_none() {
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

            if value == -1 {
                lose_positions.push(action);
            } else if value == 0 {
                draw_positions.push(action);
            } else {
                win_positions.push(action);
            }
        }

        if max_value == -1 {
            let rand_index = rand::thread_rng().gen_range(0..lose_positions.len());
            return lose_positions[rand_index];
        } else if max_value == 0 {
            let rand_index = rand::thread_rng().gen_range(0..draw_positions.len());
            return draw_positions[rand_index];
        } else {
            let rand_index = rand::thread_rng().gen_range(0..win_positions.len());
            return win_positions[rand_index];
        }
    }

    pub fn max_value(&self) -> i32 {
        let chess_result = self.terminal_test();
        let mut value = i32::min_value();

        if let Some(player) = chess_result {
            value = match player {
                Player::Max => 1,
                Player::Min => -1,
            };

            return value;
        } else if self.draw_test() {
            return 0;
        }

        for action in self.actions() {
            value = cmp::max(value, self.put(Chess::Cross, action).min_value());
        }

        value
    }

    pub fn min_value(&self) -> i32 {
        let chess_result = self.terminal_test();
        let mut value = i32::max_value();

        if let Some(player) = chess_result {
            value = match player {
                Player::Max => 1,
                Player::Min => -1,
            };

            return value;
        } else if self.draw_test() {
            return 0;
        }

        for action in self.actions() {
            value = cmp::min(value, self.put(Chess::Circle, action).max_value());
        }

        value
    }

    pub fn print(&self) {
        println!("    1   2   3 ");

        for i in 0..3 {
            print!(" {} ", i + 1);
            for j in 0..3 {
                let mut ch: char = ' ';
                if let Some(chess) = self.0[i][j] {
                    ch = match chess {
                        Chess::Circle => 'O',
                        Chess::Cross => 'X',
                    }
                }
                print!(" {:1} ", ch);
                if j != 2 {
                    print!("│");
                }
            }

            println!("");
            if i != 2 {
                println!("   ───┼───┼───");
            }
        }

        println!("");
    }
}
