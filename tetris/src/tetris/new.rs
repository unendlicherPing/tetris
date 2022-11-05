use super::{
  fields::get_fields,
  types::{Color, Rotation},
  Tetris,
};

impl Tetris {
  pub fn new() -> Tetris {
    let mut playground = [[None; 10]; 16];
    let shapes = [
      [(0, 0), (0, 1), (0, 2), (0, -1)],
      [(1, -1), (1, 0), (1, 1), (0, -1)],
      [(1, -1), (1, 0), (1, 1), (0, 1)],
      [(0, 0), (0, 1), (1, 0), (1, 1)],
      [(0, 0), (0, 1), (1, 0), (1, -1)],
      [(0, 0), (1, -1), (1, 0), (1, 1)],
      [(0, 0), (0, -1), (1, 0), (1, 1)],
    ];

    let current_shape = (shapes[2].clone(), (5, 0), Rotation::DOWN);
    let fields = get_fields(&current_shape);

    fields
      .expect("Devs fault!")
      .iter()
      .for_each(|field| playground[field.1][field.0] = Some(Color::RED));

    Tetris {
      playground,
      current_shape,
      shapes,
      game_over: false,
    }
  }
}
