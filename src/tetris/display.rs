use std::fmt::{Display, Write};

use super::Tetris;

impl Display for Tetris {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.playground.iter().for_each(|y| {
      y.iter().for_each(|x| {
        match x {
          Some(_) => {
            f.write_str("🟪 ").expect("Something went completly wrong!")
          }
          None => f.write_str("⬜ ").expect("Something went completly wrong!"),
        };
      });
      f.write_char('\n').expect("Something went completly wrong!");
    });

    Ok(())
  }
}
