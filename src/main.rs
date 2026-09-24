// use std::fs;
// use std::path::PathBuf;
// use std::process::Command;
// use std::str;


// use dioxus::prelude::*;
use ui::app::App;

mod core;
mod ui;

fn main() {
    dioxus::launch(App);
}
