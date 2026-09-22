#![allow(non_snake_case)]
use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        Title {}

        FolderSelection {}

        KeepSpectrogram {}
        
        Buttons {}
    }
}

#[component]
fn Title() -> Element {
    rsx! {
        document::Title { "Spectrographical Video Exporter" }

        // TODO document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        div { id: "title",
            h1 { "SVE" },
            p { "Spectrographical Video Exporter" }
        }
    }
}

#[component]
fn Buttons() -> Element {
    rsx! {
        div { 
            button {
                class: "btn",
                onclick: move |_| info!("buscar na pasta"),
                "Buscar na pasta" },
            button {
                class: "btn",
                onclick: move |_| info!("gerar sonogramas"),
                "Gerar sonogramas" }
        }
    }
}

#[component]
fn FolderSelection() -> Element {
    rsx! {
        input {
            type: "file",
            directory: true,
            onchange: move |evt| {
                for file in evt.files() {
                    println!("{}", file.name());
                }
            },
            "Selecionar diretório",
        }
    }
}

#[component]
fn KeepSpectrogram() -> Element {
    let mut keep_spec = use_signal(|| true);
    rsx! {
        input {
            type: "checkbox",

            // set the upload_enabled signal
            oninput: move |evt| keep_spec.set(evt.checked()),
            "Manter espectrograma"
        }
    }
}