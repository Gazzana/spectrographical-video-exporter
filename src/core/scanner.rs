use std::fs;
use std::path::PathBuf;

pub struct Caminhos {
    pub sonograma: PathBuf,
    pub audio: PathBuf,
}

/// Procura por arquivos de áudio e retorna uma lista com todos os caminhos encontrados
pub fn search(path: &str) -> Vec<Caminhos> {
    let mut caminhos: Vec<Caminhos> = Vec::new();

    // Primeira leitura
    let entradas = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return caminhos, // retorna a lista vazia caso "path" não exista
    };

    for entrada in entradas.flatten() {
        let caminho_pasta = entrada.path();

        if caminho_pasta.is_dir() {
            let mut sonograma = None;
            let mut audio = None;

            // Busca dentro de cada subpasta
            if let Ok(arquivos) = fs::read_dir(&caminho_pasta) {
                for arquivo in arquivos.flatten() {
                    let caminho_arq = arquivo.path();

                    if caminho_arq.is_file() {
                        if let Some(ext) = caminho_arq.extension() {
                            if ext == "png" {
                                sonograma = Some(caminho_arq);
                            } else if ext == "mp3" {
                                audio = Some(caminho_arq);
                            }
                        }
                    }
                }
            }
            // se encontrou ambos, dai cria o sctruct caminhos
            if let (Some(png), Some(mp3)) = (sonograma, audio) {
                caminhos.push(Caminhos {
                    sonograma: png,
                    audio: mp3,
                });
            }
        }
    }

    return caminhos;
}