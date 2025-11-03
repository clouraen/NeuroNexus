#![allow(non_snake_case)]

mod app;
mod components;
mod pages;
mod theme;
mod context;

use app::App;
use dioxus::prelude::*;

fn main() {
    launch(App);
}
