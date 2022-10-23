use std::fmt::{Display, Write};

use super::Tetris;

impl Display for Tetris {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.playground.iter().for_each(|y| {
      y.iter().for_each(|x| {
        match x {
          Some(_) => f.write_str("🟪 "),
          None => f.write_str("⬜ "),
        };
      });
      f.write_char('\n');
    });

    Ok(())
  }
}
