use self::types::{Color, Shape, ShapeState};

pub mod display;
mod fields;
pub mod input;
pub mod new;
mod test;
pub mod types;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tetris {
  playground: [[Option<Color>; 10]; 16],
  current_shape: ShapeState,
  shapes: [Shape; 7],
  game_over: bool,
}
