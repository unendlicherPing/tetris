use std::rc::Rc;

use web_sys::KeyboardEvent;
use yew::{html, Component, Context};
use yewdux::{prelude::Dispatch, store::Reducer};

use crate::tetris::types::Input;

use super::global::State;

pub enum Msg {
  KeyBoardEvent(KeyboardEvent),
  Update(Rc<State>),
}

impl Reducer<State> for Msg {
  fn apply(&self, mut state: std::rc::Rc<State>) -> std::rc::Rc<State> {
    let game = Rc::make_mut(&mut state);

    game.game.input(Input::DOWN);

    state
  }
}

pub struct App {
  dispatch: Dispatch<State>,
  game: Rc<State>,
}

impl Component for App {
  type Message = Msg;
  type Properties = ();

  fn create(ctx: &Context<Self>) -> Self {
    // The callback for receiving updates to state.
    let callback: _ = ctx.link().callback(Msg::Update);
    // Subscribe to changes in state. New state is received in `update`. Be sure to save this,
    // dropping it will unsubscribe.
    let dispatch = Dispatch::<State>::subscribe(callback);
    Self {
      // Get the current state.
      game: dispatch.get(),
      dispatch,
    }
  }

  fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      // Receive new state.
      Msg::Update(game) => {
        self.game = game;
      }
      Msg::KeyBoardEvent(event) => match event.key().as_ref() {
        "w" => {
          self
            .dispatch
            .reduce_mut(|state| state.game.input(Input::UP));
        }
        "s" => {
          self
            .dispatch
            .reduce_mut(|state| state.game.input(Input::DOWN));
        }
        "a" => {
          self
            .dispatch
            .reduce_mut(|state| state.game.input(Input::LEFT));
        }
        "d" => {
          self
            .dispatch
            .reduce_mut(|state| state.game.input(Input::RIGHT));
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

    html! {
      <>
        <div> { self.dispatch.get().game.to_string() } </div>
        <input type="text" {onkeypress} />
      </>
    }
  }
}
