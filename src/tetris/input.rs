use super::{
  types::{Color, Input, Rotation},
  util::get_fields,
  Tetris,
};

impl Tetris {
  pub fn input(&mut self, input: Input) {
    {
      let fields = get_fields(&self.current_shape);

      fields
        .iter()
        .for_each(|field| self.playground[field.1][field.0] = None);
    }

    match input {
      Input::DOWN => {
        self.current_shape.1 .1 += 1;
      }

      Input::LEFT => {
        self.current_shape.1 .0 -= 1;
      }

      Input::RIGHT => {
        self.current_shape.1 .0 += 1;
      }

      Input::UP => {
        self.current_shape.2 = match self.current_shape.2 {
          Rotation::UP => Rotation::RIGHT,
          Rotation::RIGHT => Rotation::DOWN,
          Rotation::DOWN => Rotation::LEFT,
          Rotation::LEFT => Rotation::UP,
        };
      }
    }

    let fields = get_fields(&self.current_shape);

    fields
      .iter()
      .for_each(|field| self.playground[field.1][field.0] = Some(Color::RED));
  }
}
