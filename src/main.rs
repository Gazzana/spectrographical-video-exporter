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

// TODO: #[allow(unused)] temporario
#[allow(unused)]
fn init() {
    let paths = core::scanner::search("./samples");

    for i in paths {
        println!("Executando em: {:?}", i);

        core::renderer::generate_sonogram(i, false);
    }
}


