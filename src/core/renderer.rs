use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;
use tempfile::{Builder, TempPath};

/// Deve ser implementado de forma assincrona, uma instancia para cada sonograma.
///
/// 1. Define o caminho do espectrograma de acordo com o bool keep_img
///
/// 3. Cria o espectrograma cinza com SoX
///
/// 4. Aplica o filtro de cores desejado -------:
///                                             :- Usando o mesmo comando para otimizacao
/// 5. Cria o filtro de agulha de reproducao ---:
///
/// 6. Cria o video, usando o filtro, somando imagem + som + filtro de agulha
///
/// 7. Fim da função, aqui o possivel espectrograma temporario deve ser deletado
// pub fn generate_sonogram(audio_path: PathBuf, keep_img: bool, cor_agulha: &str) -> bool {
//
//
//     let mut _arquivo_temp = None;
//
//     // 1: Define o caminho do espectrograma de acordo com o bool keep_img
//
//     let img_path = if keep_img {
//         audio_path.with_extension("png")
//     } else {
//         let temp = Builder::new()
//             .suffix("png")
//             .tempfile()
//             .expect("Falha ao criar arquivo temporario");
//
//         let caminho = temp.path().to_path_buf();
//         _arquivo_temp = Some(temp); // Ancora o arquivo na função
//         caminho
//     };
//
//     let video_path = audio_path.with_extension("mp4");
//
//     // CONVERSAO PARA WAV
//     let wav_temp = Builder::new().suffix(".wav").tempfile().unwrap();
//     let wav_path = wav_temp.path().to_path_buf();
//
//     // Usa o FFmpeg para decodificar o MP3 para WAV silenciosamente
//     let decodificacao = Command::new("ffmpeg")
//     .args(&[
//         "-i", audio_path.to_str().unwrap(), // O arquivo MP3, FLAC, etc
//         "-y", // Sobrescreve sem perguntar
//         wav_path.to_str().unwrap(),
//     ])
//     .output();
//
//     // 2:
//
//     // sox sample.wav -n remix - spectrogram -x 1920 -Y 1080 -r -o sampleout.png
//
//     let sox_status = Command::new("sox")
//         .args(&[
//             wav_path.to_str().unwrap(),
//             "-n",
//             "remix",
//             "-", // Downmix estéreo para mono
//             "spectrogram",
//             "-x",
//             "1920", // Largura exata
//             "-r",   // Sem bordas
//             "-o",
//             img_path.to_str().unwrap(),
//         ])
//         .status()
//         .expect("Falha ao invocar o SoX");
//
//     if !sox_status.success() {
//         eprintln!("SoX: Erro em {:?}", audio_path);
//         return false;
//     }
//
//     // 3.
//
//     let ffprobe_output = Command::new("ffprobe")
//         .args(&[
//             "-v",
//             "error",
//             "-show_entries",
//             "format=duration",
//             "-of",
//             "default=noprint_wrappers=1:nokey=1",
//             audio_path.to_str().unwrap(),
//         ])
//         .output()
//         .expect("Falha ao executar o ffprobe");
//
//     let duracao_str = str::from_utf8(&ffprobe_output.stdout).unwrap().trim();
//
//     // A largura fixa da tela é 1920. A agulha anda de 0 a 1920 dividindo pelo tempo.
//     let filtro = format!(
//         "[1:v]scale=1920:1080[bg];[bg][2:v]overlay=x='(1920/{})*t':y=0:shortest=1[video]",
//         duracao_str
//     );
//
//     // Definindo a cor da agulha
//     let input_agulha = format!("color=c={}:s=3x1080", cor_agulha);
//
//     let ffmpeg_status = Command::new("ffmpeg")
//         .args(&[
//             "-i", audio_path.to_str().unwrap(),
//             "-loop", "1", "-framerate", "24", "-i", img_path.to_str().unwrap(),
//             "-f", "lavfi", "-i", &input_agulha,
//             "-filter_complex", &filtro,
//             "-map", "[video]",
//             "-map", "0:a",
//             "-c:v", "libx264",
//             "-preset", "fast",
//             "-pix_fmt", "yuv420p",
//             "-c:a", "copy", // Copia o áudio original 1:1
//             "-shortest",
//             "-y",
//             video_path.to_str().unwrap(),
//         ])
//         .status()
//         .expect("Falha ao invocar o ffmpeg");
//
//     // TODO: futuramente remover esses println
//     if ffmpeg_status.success() {
//         println!("Vídeo finalizado: {:?}", video_path.display());
//     } else {
//         eprintln!("Erro na renderização do vídeo.");
//         return false;
//     }
//
//     // 4.
//
//     true
// }

pub fn generate_spectrogram(audio_path: &Path, keep_img: bool) -> (PathBuf, Option<TempPath>) {
    // Definir o caminho do espectrograma de acordo com a bool keep_img
    let (img_path, ancora_img) = if keep_img {
        (audio_path.with_extension("png"), None) // nome_da_musica.mp3 -> nome_da_musica.png
    } else { // no caso de arquivo temporario
        let temp = Builder::new()
            .suffix(".png")
            .tempfile()
            .unwrap();
        let temp_path = temp.into_temp_path();
        let caminho = temp_path.to_path_buf();
            (caminho, Some(temp_path))
    };

    // Conversao silenciosa para wavefile
    let wav_temp = Builder::new().suffix(".wav").tempfile().expect("Falha ao criar WAV temporario");
    let wav_path = wav_temp.into_temp_path();
    let wav_path_buf = wav_path.to_path_buf();

    let converted_wav = Command::new("ffmpeg")
        .args(&[
            "-i", audio_path.to_str().unwrap(),
            "-y",
            wav_path_buf.to_str().unwrap(),
        ])
        .status();

    if converted_wav.is_err() || !converted_wav.unwrap().success() {
        eprintln!("FFmpeg: Erro ao converter o áudio para WAV temporário.");
    }

    // Criaçao do espectrograma cinza com SoX
    // sox sample.wav -n remix - spectrogram -m -x 1920 -Y 1080 -r -o sampleout.png
    let sox_status = Command::new("sox")
        .args(&[
            wav_path.to_str().unwrap(),
            "-n",
            "remix",
            "-",
            "spectrogram",
            "-m",
            "-x",
            "1920",
            "-Y",
            "1080",
            "-r",
            "-o",
            img_path.to_str().unwrap(),
        ])
        .status()
        .expect("Falha ao invocar o SoX");

    if !sox_status.success() {
        eprintln!("SoX: Erro em {:?}", audio_path);
    }


    (img_path, ancora_img)
}

pub fn generate_sonogram(audio_path: &Path, img_path: &Path, preset_cor: &str, cor_agulha: &str,) -> bool {
    let video_path = audio_path.with_extension("mp4");

    let ffprobe_output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            audio_path.to_str().unwrap(),
        ])
        .output();

    let output = match ffprobe_output {
        Ok(out) => out,
        Err(_) => {
            eprintln!("Erro: Falha ao executar o ffprobe.");
            return false;
        }
    };

    let duracao_str = str::from_utf8(&output.stdout).unwrap_or("").trim();

    if duracao_str.is_empty() || duracao_str.parse::<f64>().is_err() {
        eprintln!("Erro: ffprobe não conseguiu ler a duração do áudio em {:?}", audio_path);
        return false;
    }


    let agulha = format!("color=c={}:s=3x1080", cor_agulha);
    let filtro = format!(
        "[1:v]pseudocolor=preset={},scale=1920:1080[fundo_colorido];[fundo_colorido][2:v]overlay=x='(1920/{})*t':y=0:shortest=1[video]",
        preset_cor, duracao_str
    );

    // Renderizar video final
    let ffmpeg_status = Command::new("ffmpeg").args([&
        "-i", audio_path.to_str().unwrap(),
        "-loop", "1", "-framerate", "24", "-i", img_path.to_str().unwrap(),
        "-f", "lavfi", "-i", &agulha,
        "-filter_complex", &filtro,
        "-map", "[video]",
        "-map", "0:a",
        "-c:v", "libx264",
        "-preset", "fast",
        "-pix_fmt", "yuv420p",
        "-c:a", "copy", // copia 1:1
        "-shortest",
        "-y",
        video_path.to_str().unwrap(),
    ])
    .status();

    match ffmpeg_status {
        Ok(status) => {
            if status.success() {
                println!("Vídeo renderizado com sucesso: {:?}", video_path);
                true
            } else {
                eprintln!("Erro interno no FFmpeg ao renderizar {:?}", video_path);
                false
            }
        },
        Err(e) => {
            eprintln!("Falha ao tentar invocar o FFmpeg: {}", e);
            false
        }
    }
}

pub fn call_renderer(audio_path: PathBuf, keep_img: bool, preset: &str, cor_agulha: &str) -> bool {
    let (caminho_img, _ancora) = generate_spectrogram(&audio_path, keep_img);

    // 2. Verifica se a imagem realmente foi criada (SoX não falhou)
    if !caminho_img.exists() {
        return false;
    }

    let sonogram_status = generate_sonogram(&audio_path, &caminho_img, preset, cor_agulha);

    sonogram_status
}
