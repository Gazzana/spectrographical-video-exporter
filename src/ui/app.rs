#![allow(non_snake_case)]
use std::path::PathBuf;

use dioxus::prelude::*;

static CSS: Asset = asset!("/assets/main.css");

#[component]
pub fn App() -> Element {

    let color_preset = use_signal(|| String::from("magma"));
    let cor_agulha = use_signal(|| String::from("#FF0000"));
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
            color_preset,
            cor_agulha,
            audio_files,
            status_msg,
            is_processing
        }

        AgulhaColorSelector { cor_agulha }
        SpectrogramPresetSelection { color_preset }

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
    color_preset: Signal<String>,
    cor_agulha: Signal<String>,
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

                    let preset_escolhido = color_preset.read().clone();
                    let cor_escolhida_agulha = cor_agulha.read().clone();

                    spawn(async move {
                        let total = arquivos.len();
                        let mut sucessos = 0;

                        // Itera sobre a lista de músicas encontrada
                        for (i, arquivo) in arquivos.into_iter().enumerate() {
                            let nome = arquivo.file_name().unwrap_or_default().to_string_lossy();
                            status_msg.set(format!("Renderizando {}/{} ({})...", i + 1, total, nome));

                            // Clonando as variaveis para thread assincrona
                            let arquivo_clone = arquivo.clone();
                            let manter_clone = manter_img;

                            let preset_clone = preset_escolhido.clone();
                            let agulha_clone = cor_escolhida_agulha.clone();

                            let sucesso = tokio::task::spawn_blocking(move || {
                                crate::core::renderer::call_renderer(arquivo_clone, manter_clone, &preset_clone, &agulha_clone)
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


const PRESETS_FFMPEG: &[&str] = &["magma", "inferno", "plasma", "viridis", "turbo", "cividis", "range1", "range2", "shadows", "highlights", "solar", "nominal", "preferred", "total", "spectral", "cool", "heat", "fiery", "blues", "green", "helix"];

#[component]
fn SpectrogramPresetSelection(mut color_preset: Signal<String>) -> Element {
    rsx! {
        div { class: "preset-selector-container",
            label { margin_right: "10px", "Cores do Sonograma:" }
            select {
                // Mostra o valor atual armazenado no Signal
                value: "{color_preset}",
                // Atualiza o estado global quando o usuário escolhe outra opção
                onchange: move |evt| color_preset.set(evt.value()),

                // O Dioxus gera automaticamente as tags <option> baseadas na nossa array
                for preset in PRESETS_FFMPEG {
                    option {
                        value: "{preset}",
                        // Coloca a primeira letra em maiúscula
                        "{preset[0..1].to_uppercase()}{&preset[1..]}"
                    }
                }
            }
        }
    }
}



#[component]
fn AgulhaColorSelector(mut cor_agulha: Signal<String>) -> Element {
    rsx! {
        input {
            type: "color",
            value: "{cor_agulha}",
            oninput: move |evt| cor_agulha.set(evt.value())
        }
    }
}

// TODO: remover essa palhacada
#[component]
fn StatusPanel(status_msg: Signal<String>, audio_files: Signal<Vec<PathBuf>>) -> Element {
    rsx! {
        div { class: "status-panel",
            hr {}
            h3 { "Status [debug-only]" }
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
