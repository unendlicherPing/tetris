use super::types::{Position, Rotation, ShapeState};

pub fn get_fields((shape, (x, y), rotaton): &ShapeState) -> [Position; 4] {
  let mut fields = [(0usize, 0usize); 4];

  for i in 0..4 {
    let sx = &shape[i].0;
    let sy = &shape[i].1;
    let rotated_vec = match rotaton {
      Rotation::DOWN => shape[i],
      Rotation::UP => (-*sx, -*sy),
      Rotation::RIGHT => (*sy, -*sx),
      Rotation::LEFT => (-*sy, *sx),
    };

    fields[i] = (
      (x.to_owned() as isize + &rotated_vec.1) as usize,
      (y.to_owned() as isize + &rotated_vec.0) as usize,
    );
  }

  fields
}
