#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};



mod item;
mod modal;
mod input;

use crate::item::Item;
use crate::modal::Modal;

const KEY: &'static str = "yew.rust.crud.database";

pub struct Model {
  storage: StorageService,
  state: List,
  link: ComponentLink<Self>
}

pub struct List {
    items: Vec<Item>,
    modal_visible: bool,
    current_item: Option<Item>
  }

  pub enum Msg {
    New,
    HiddedModal,
    Saved(Item),
    Edit(usize),
    Remove(usize),
    Store
  }