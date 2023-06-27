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

  impl Component for Model {
    type Message = Msg;
    type Properties = ();
  
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let storage = StorageService::new(Area::Local).expect("Storage Error");
    
        let items = {
          if let Json(Ok(items)) = storage.restore(KEY) {
            items
          } else {
            Vec::new()
          }
        };
    
        let state = List {
          items,
          modal_visible: false,
          current_item: None
        };
    
        Model { storage, state, link }
      }

      fn update(&mut self, msg: Self::Message) -> ShouldRender {

        match msg {
          Msg::New => {
            self.state.modal_visible = true;
            self.state.current_item = None;
    
            true
          }
    
          
        }
      }
  
  }