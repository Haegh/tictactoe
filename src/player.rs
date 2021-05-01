//! Tic tac toe player of the game module.

/// Player structure
#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: u8,
    pub symbol: String,
}

impl Player {
    /// Create a new player
    /// 
    /// Example
    /// 
    /// ```
    /// let player = Player::new(1, String::from("O"))
    /// ```
    pub fn new(id: u8, symbol: String) -> Player {
        Player {
            id: id,
            symbol: symbol,
        }
    }
}


#[test]
fn test_new() {
    let player = Player::new(1, String::from("O"));
    let player_2 = Player {id: 1, symbol: String::from("O")};
    assert_eq!(player, player_2);
}
