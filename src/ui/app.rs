#![allow(non_snake_case)]
use std::path::PathBuf;

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {

    let folder_path = use_signal(|| String::new());
    let keep_img = use_signal(|| false);
    let audio_files = use_signal(|| Vec::<PathBuf>::new());

    // Signals para feedback do useuario (temporario)
    let status_msg = use_signal(|| String::from("Aguardando ação..."));
    let is_processing = use_signal(|| false);

    rsx! {
        document::Stylesheet { href: CSS }

        Title {}

        FolderSelector { folder_path }

        KeepSpectrogram { keep_img }
        
        Buttons {
            folder_path,
            keep_img,
            audio_files,
            status_msg,
            is_processing
        }

        StatusPanel { status_msg, audio_files }
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
fn Buttons(
    folder_path: Signal<String>,
    keep_img: Signal<bool>,
    mut audio_files: Signal<Vec<PathBuf>>,
    mut status_msg: Signal<String>,
    mut is_processing: Signal<bool>
) -> Element {
    rsx! {
        div { 
            // Botao de buscar arvquivos
            button {
                class: "btn",
                disabled: is_processing(),
                onclick: move |_| {
                    let path = folder_path.read().clone();
                    if path.is_empty() {
                        status_msg.set("Erro: Digite o caminho de uma pasta primeiro.".to_string());
                        return;
                    }
                    
                    status_msg.set("Buscando arquivos de áudio...".to_string());

                    // CHAMA SEU BACKEND AQUI
                    let encontrados = crate::core::scanner::search(&path);
                    
                    let qtd = encontrados.len();
                    
                    audio_files.set(encontrados);
                    status_msg.set(format!("Busca concluída: {} arquivo(s) encontrado(s).", qtd));
                },
                "Buscar na pasta" },
            // Botao de gerar sonogramas
            button {
                class: "btn",
                // Desabilita o botão se já estiver processando ou se não houver músicas
                disabled: is_processing() || audio_files.read().is_empty(),
                onclick: move |_| {
                    is_processing.set(true);
                    
                    let arquivos = audio_files.read().clone();
                    let manter_img = keep_img();

                    spawn(async move {
                        let total = arquivos.len();
                        let mut sucessos = 0;

                        // Itera sobre a lista de músicas encontrada
                        for (i, arquivo) in arquivos.into_iter().enumerate() {
                            let nome = arquivo.file_name().unwrap_or_default().to_string_lossy();
                            status_msg.set(format!("Renderizando {}/{} ({})...", i + 1, total, nome));
                            
                            let arquivo_clone = arquivo.clone();
                            let manter_clone = manter_img;
                            
                            let sucesso = tokio::task::spawn_blocking(move || {
                                crate::core::renderer::generate_sonogram(arquivo_clone, manter_clone)
                            }).await.unwrap();

                            if sucesso {
                                sucessos += 1;
                            }
                        }

                        status_msg.set(format!("Lote concluído! {}/{} vídeos gerados com sucesso.", sucessos, total));
                        is_processing.set(false);
                    });
                },
                "Gerar sonogramas"
            }
        }
    }
}

#[component]
fn FolderSelector(mut folder_path: Signal<String>) -> Element {
    rsx! {
        div {
            button {
                class: "btn",
                onclick: move |_| {
                    // Janela nativa de selecao de pastas
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        folder_path.set(path.to_string_lossy().to_string());
                    }
                },
                "Selecionar diretorio"
            }
            // Mostra na tela a pasta que o usuário escolheu
            span { margin_left: "10px", "{folder_path}" }
        }
    }
}

#[component]
fn KeepSpectrogram(mut keep_img: Signal<bool>) -> Element {
    rsx! {
        input {
            type: "checkbox",
            checked: "{keep_img}",
            // set the upload_enabled signal
            oninput: move |evt| keep_img.set(evt.checked()),
        }
        label { "Manter espectrograma" }
    }
}

// TODO: remover essa palhacada
#[component]
fn StatusPanel(status_msg: Signal<String>, audio_files: Signal<Vec<PathBuf>>) -> Element {
    rsx! {
        div { class: "status-panel",
            hr {}
            h3 { "Painel de Status" }
            p { font_weight: "bold", color: "#e74c3c", "{status_msg}" }
            
            // Mostra o que encontrou na pasta
            if !audio_files.read().is_empty() {
                ul {
                    for arquivo in audio_files.read().iter() {
                        li { "{arquivo.file_name().unwrap_or_default().to_string_lossy()}" }
                    }
                }
            }
        }
    }
}