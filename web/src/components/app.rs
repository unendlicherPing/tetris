use web_sys::KeyboardEvent;
use yew::{html, Component, Context, Html};

use crate::components::row::Row;
use tetris::tetris::{types::Input, Tetris};

pub enum Msg {
  KeyBoardEvent(KeyboardEvent),
}

pub struct App {
  game: Tetris,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(_ctx: &Context<Self>) -> Self {
    Self {
      game: Tetris::new(),
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::KeyBoardEvent(event) => match event.key().as_ref() {
        "w" => {
          self.game.input(Input::UP);
        }
        "s" => {
          self.game.input(Input::DOWN);
        }
        "a" => {
          self.game.input(Input::LEFT);
        }
        "d" => {
          self.game.input(Input::RIGHT);
        }
        _ => {}
      },
    }

    true
  }

  fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
    let onkeypress = ctx
      .link()
      .batch_callback(|event: KeyboardEvent| Some(Msg::KeyBoardEvent(event)));

    let rows = self
      .game
      .to_string()
      .split('\n')
      .map(|row| {
        html! {
          <Row content={ row.to_owned() } />
        }
      })
      .collect::<Html>();

    html! {
      <>
        <table> { rows } </table>
        <input type="text" {onkeypress} />
      </>
    }
  }
}
