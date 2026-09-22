use std::fs;
use std::path::{Path, PathBuf};

/// Procura por arquivos de áudio e retorna uma lista com todos os caminhos encontrados
pub fn search<P: AsRef<Path>>(diretorio: P) -> Vec<PathBuf> {
    let mut audios_encontrados: Vec<PathBuf> = Vec::new();

    // Lista de formatos suportados pelo SoX e FFmpeg
    let ext_validas = ["mp3", "wav", "flac", "ogg", "aiff", "m4a"];

    if let Ok(entradas) = fs::read_dir(diretorio) {
        for entrada in entradas.flatten() {
            let caminho = entrada.path();

            if caminho.is_dir() {
                // Se achou uma subpasta, a função chama a si mesma recursivamente
                let mut sub_audios = search(&caminho);
                audios_encontrados.append(&mut sub_audios);
            } else if caminho.is_file() {
                if let Some(extensao) = caminho.extension() {
                    if let Some(ext_str) = extensao.to_str() {
                        if ext_validas.contains(&ext_str.to_lowercase().as_str()) {
                            audios_encontrados.push(caminho);
                        }
                    }
                }
            }
        }
    }
    return audios_encontrados;
}
