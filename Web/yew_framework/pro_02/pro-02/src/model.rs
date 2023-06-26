#![recursion_limit = "512"]

use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::format::Json;
use yew::services::storage::{Area, StorageService};



mod item;
mod modal;
mod input;

use crate::item::Item;
use crate::modal::Modal;