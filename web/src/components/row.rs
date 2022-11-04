use crate::components::item::Item;
use yew::{function_component, html, Html, Properties};

#[derive(PartialEq, Properties)]
pub struct RowProperties {
  pub content: String,
}

#[function_component(Row)]
pub fn row(RowProperties { content }: &RowProperties) -> Html {
  let items = content
    .split(" ")
    .map(|item| {
      html! {
        <Item item_data={ item.to_owned() } />
      }
    })
    .collect::<Html>();

  html! {
    <tr> { items } </tr>
  }
}
