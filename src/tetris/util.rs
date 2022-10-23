use super::types::{
  OutOfBoundsException, Position, Rotation, ShapeState, VecVal,
};

pub fn get_fields(
  (shape, (x, y), rotaton): &ShapeState,
) -> Result<[Position; 4], OutOfBoundsException> {
  let mut fields = [(0usize, 0usize); 4];

  for i in 0..4 {
    let shape_x = &shape[i].0;
    let shape_y = &shape[i].1;
    let rotated_vec = match rotaton {
      Rotation::DOWN => shape[i],
      Rotation::UP => (-*shape_x, -*shape_y),
      Rotation::RIGHT => (*shape_y, -*shape_x),
      Rotation::LEFT => (-*shape_y, *shape_x),
    };

    let shape_pos = add_vec_on_pos(&(x.to_owned(), y.to_owned()), &rotated_vec);

    match shape_pos {
      Ok(pos) => fields[i] = pos,
      Err(err) => return Err(err),
    };
  }

  Ok(fields)
}

fn add_vec_on_pos(
  (x, y): &Position,
  vec: &VecVal,
) -> Result<Position, OutOfBoundsException> {
  let new_x = x.to_owned() as isize + &vec.1;
  let new_y = y.to_owned() as isize + &vec.0;

  let x_oob = new_x < 0 || new_x >= 10;
  let y_oob = new_y < 0 || new_y >= 16;

  if x_oob || y_oob {
    return Err(OutOfBoundsException {});
  }

  let new_pos = (new_x as usize, new_y as usize);
  Ok(new_pos)
}
