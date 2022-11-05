use super::{
  fields::get_fields,
  grounded::is_grounded,
  types::{Color, Input, Rotation, ShapeState},
  Tetris,
};

impl Tetris {
  fn clear(&mut self) {
    let fields = get_fields(&self.current_shape);

    match fields {
      Ok(val) => {
        val
          .iter()
          .for_each(|field| self.playground[field.1][field.0] = None);
      }
      Err(_) => {
        return;
      }
    }
  }

  fn render(&mut self) {
    let fields = get_fields(&self.current_shape).expect("Devs fault!");
    fields
      .iter()
      .for_each(|field| self.playground[field.1][field.0] = Some(Color::RED));
  }

  pub fn input(&mut self, input: Input) {
    let new_shape: ShapeState = match input {
      Input::DOWN => {
        let mut shape = self.current_shape.clone();
        shape.1 .1 += 1;
        shape
      }

      Input::LEFT => {
        let mut shape = self.current_shape.clone();
        shape.1 .0 -= 1;
        shape
      }

      Input::RIGHT => {
        let mut shape = self.current_shape.clone();
        shape.1 .0 += 1;
        shape
      }
      Input::UP => {
        let mut shape = self.current_shape.clone();
        shape.2 = match shape.2 {
          Rotation::UP => Rotation::RIGHT,
          Rotation::RIGHT => Rotation::DOWN,
          Rotation::DOWN => Rotation::LEFT,
          Rotation::LEFT => Rotation::UP,
        };
        shape
      }
    };

    match get_fields(&new_shape) {
      Ok(_) => {
        self.clear();
        self.current_shape = new_shape;
      }
      Err(_) => return,
    }

    self.render();

    // // if is_grounded(
    //   &self.playground,
    //   &get_fields(&self.current_shape).expect("Devs fault!"),
    // ) {
    //   self.current_shape = (self.shapes[1].clone(), (5, 0), Rotation::DOWN);
    // }
  }
}
