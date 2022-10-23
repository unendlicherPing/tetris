use yewdux::store::Store;

use crate::tetris::Tetris;

#[derive(Debug, Clone, PartialEq, Eq, Store)]
pub struct State {
  pub game: Tetris,
}

impl Default for State {
  fn default() -> Self {
    Self {
      game: Tetris::new(),
    }
  }
}
