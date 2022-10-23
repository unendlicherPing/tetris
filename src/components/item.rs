use yew::{function_component, html, Properties};

#[derive(PartialEq, Properties)]
pub struct ItemProperties {
  pub item_data: String,
}

#[function_component(Item)]
pub fn item(ItemProperties { item_data }: &ItemProperties) -> Html {
  html! {
    <td> { item_data } </td>
  }
}
