use yew::{html, Component, ComponentLink, Html, ShouldRender, Callback, Properties};


use yew::services::{
    ConsoleService
  };

  #[derive(Properties, Clone)]
pub struct ModalProperties {
  pub item: Item,
  pub visible: bool,
  pub on_close: Callback<bool>,
  pub on_save: Callback<Item>
}

pub struct Modal {
    pub item: Item,
    pub name: String,
    pub price: String,
    pub visible: bool,
    pub on_close: Callback<bool>,
    pub on_save: Callback<Item>,
    error: Option<Vec<ItemValidationErr>>,
    link: ComponentLink<Self>
  }

  pub enum ModalMsg {
    HideModal,
    SetName(String),
    SetPrice(String),
    Save
  }