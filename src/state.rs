//! Tic tac toe state of the game module.

/// State of the game enum
#[derive(Clone, Debug, PartialEq)]
pub enum State {
    Start,
    End,
}
