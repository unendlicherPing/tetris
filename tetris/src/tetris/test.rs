#[cfg(test)]
mod tests {
  use crate::tetris::{types::Input, Tetris};

  #[test]
  fn test() {
    let mut tetris = Tetris::new();
    println!("{}", tetris);

    tetris.input(Input::DOWN);
    println!("{}", tetris);

    tetris.input(Input::DOWN);
    println!("{}", tetris);

    tetris.input(Input::DOWN);
    println!("{}", tetris);

    tetris.input(Input::UP);
    println!("{}", tetris);

    tetris.input(Input::UP);
    println!("{}", tetris);

    tetris.input(Input::UP);
    println!("{}", tetris);

    tetris.input(Input::UP);
    println!("{}", tetris);
  }
}
