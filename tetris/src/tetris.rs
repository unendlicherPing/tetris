use self::types::{Shape, ShapeState, Map};

pub mod display;
mod fields;
mod grounded;
pub mod input;
pub mod new;
mod test;
pub mod types;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Tetris {
  playground: Map,
  current_shape: ShapeState,
  shapes: [Shape; 7],
  game_over: bool,
}
