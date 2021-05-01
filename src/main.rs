#[derive(Clone, Debug, PartialEq)]
enum State {
    Start,
    End,
}

#[derive(Clone, Debug)]
struct Board {
    cases: Vec<String>,
    players: Vec<Player>,
    player_turn: u8,
    state: State,
}

#[derive(Clone, Debug)]
struct Player {
    id: u8,
    symbol: String,
}

impl Board {
    fn new(players: Vec<Player>) -> Board {
        Board {
            cases: vec![String::from(" "); 9],
            players: players,
            player_turn: 1,
            state: State::Start,
        }
    }

    fn new_turn(&mut self) {
        if self.player_turn == self.players[0].id {
            self.player_turn = self.players[1].id;
        } else {
            self.player_turn = self.players[0].id;
        }
    }

    fn ending_check(&mut self) {
        let wins: Vec<(usize, usize, usize)> = vec![
            (0, 1, 2), (3, 4, 5), (6, 7, 8),
            (0, 3, 6), (1, 4, 7), (2, 5, 8),
            (0, 4, 8), (2, 4, 6),
        ];
        for i in wins.iter() {
            if self.cases[i.0] == self.cases[i.1] && self.cases[i.0] == self.cases[i.2] && self.cases[i.0] != " " {
                self.state = State::End;
                println!("Game finished.");
                break;
            }
        }
    }

    fn play(&mut self) {
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
                self.cases[number] = player.symbol.clone();
                break;
            } else {
                println!("This is not a number.");
                continue;
            }
        }
    }

    fn show(&self) {
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

impl Player {
    fn new(id: u8, symbol: String) -> Player {
        Player {
            id: id,
            symbol: symbol,
        }
    }
}


fn main() {
    let player_list: Vec<Player> = vec![
        Player::new(1, String::from("O")), 
        Player::new(2, String::from("X")),
    ];
    let mut game: Board = Board::new(player_list);

    game.show();
    while game.state == State::Start {
        game.play();
        game.show();
        game.ending_check();
        game.new_turn();
    };
}
