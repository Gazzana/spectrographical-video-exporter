// use std::fs;
// use std::path::PathBuf;
// use std::process::Command;
// use std::str;


// use dioxus::prelude::*;
use ui::app::App;

mod core;
mod ui;

// sox sample.wav -n remix - spectrogram -x 1920 -Y 1080 -r -o sampleout.png

fn main() {
    dioxus::launch(App);
}

// TODO TEMPORARIO
#[allow(unused)]
fn init() {
    let paths = core::scanner::search("./samples");

    for i in paths {
        println!("Executando em:");
        println!("Áudio: {:?}", i.audio);
        println!("Imagem: {:?}", i.sonograma);

        core::renderer::generate_sonogram(i.audio, i.sonograma);
    }
}


