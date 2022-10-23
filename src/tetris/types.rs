#[derive(Debug)]
pub enum Input {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Color {
  RED,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Rotation {
  UP,
  DOWN,
  LEFT,
  RIGHT,
}

pub type Position = (usize, usize);
pub type VecVal = (isize, isize);
pub type Shape = [VecVal; 4];
pub type ShapeState = (Shape, Position, Rotation);

#[derive(Debug)]
pub struct OutOfBoundsException;
