//! Tic tac toe board game module.
use crate::player::Player;
use crate::state::State;


/// Board structure
#[derive(Clone, Debug, PartialEq)]
pub struct Board {
    pub cases: Vec<String>,
    pub players: Vec<Player>,
    pub player_turn: u8,
    pub state: State,
}

impl Board {
    /// Create a new board
    /// 
    /// Example
    /// 
    /// ```
    /// let player_list = vec![Player::new(1, String::from("O")),  Player::new(2, String::from("X"))];
    /// let game = Board::new(player_list);
    /// ```
    pub fn new(players: Vec<Player>) -> Board {
        Board {
            cases: vec![String::from(" "); 9],
            players: players,
            player_turn: 1,
            state: State::Start,
        }
    }

    /// Change the actual player turn
    pub fn new_turn(&mut self) {
        if self.player_turn == self.players[0].id {
            self.player_turn = self.players[1].id;
        } else {
            self.player_turn = self.players[0].id;
        }
    }

    /// Control if a player as win
    pub fn ending_check(&mut self) {
        // Control if there is empty cases
        let mut empty_cases: bool = false; 
        for case in self.cases.iter() {
            if case == &String::from(" ") {
                empty_cases = true;
            }
        }
        if empty_cases == false {
            self.state = State::End;
            println!("Game finished.");
            println!("Nobody win.");
        }
        
        // Control all the win possibilities
        let wins: Vec<(usize, usize, usize)> = vec![
            (0, 1, 2), (3, 4, 5), (6, 7, 8),
            (0, 3, 6), (1, 4, 7), (2, 5, 8),
            (0, 4, 8), (2, 4, 6),
        ];
        for i in wins.iter() {
            if self.cases[i.0] == self.cases[i.1] && self.cases[i.0] == self.cases[i.2] && self.cases[i.0] != " " {
                self.state = State::End;
                println!("Game finished.");
                println!("Player {} win.", self.player_turn);
                break;
            }
        }
    }

    /// Ask the player to play and control the input data
    pub fn play(&mut self) {
        let player = self.players.iter().find(|&x| x.id == self.player_turn).unwrap();

        println!("Player {} turn.", player.id);
        loop {
            println!("Enter a number : ");
            let mut input: String = String::new();
            if std::io::stdin().read_line(&mut input).is_err() {
                println!("Couldn't read line! Try again.");
                continue;
            }
            if let Ok(number) = input.trim().parse::<usize>() {
                if number < 1 || number > 9 {
                    println!("The field number must be between 1 and 9.");
                    continue;
                }
                let number = number - 1;
                if self.cases[number] != String::from(" ") {
                    println!("The field must not be already used.");
                    continue;
                }
                self.cases[number] = player.symbol.clone();
                break;
            } else {
                println!("This is not a number.");
                continue;
            }
        }
    }

    /// Show the game board
    pub fn show(&self) {
        let mut string_show: String = String::from("|");
        for i in 0..9 {
            let tmp: String = format!("{}|", self.cases[i]);
            string_show = format!("{}{}", string_show, tmp);
            if i == 2 || i == 5 || i == 9 {
                string_show = format!("{}{}|", string_show, "\n");
            }
        }
        println!("\n{}\n", string_show);
    }
}


#[test]
fn test_new() {
    let player_list: Vec<Player> = vec![
        Player::new(1, String::from("O")), 
        Player::new(2, String::from("X")),
    ];
    let game = Board::new(player_list.clone());
    let game_2 = Board {
        cases: vec![String::from(" "); 9],
        players: player_list,
        player_turn: 1,
        state: State::Start,
    };
    assert_eq!(game, game_2);
}
