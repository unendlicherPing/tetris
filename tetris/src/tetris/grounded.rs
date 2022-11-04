use super::types::{Map, Position};

pub fn is_grounded(map: &Map, positions: &[Position; 4]) -> bool {
  for (y, x) in positions {
    if y.to_owned() == 16
      || match map[y.to_owned()][x.to_owned() - 1] {
        Some(_) => true,
        None => false,
      }
    {
      return true;
    }
  }

  false
}
