use std::path::PathBuf;
use std::process::Command;
use std::str;
use tempfile::Builder;

/// Deve ser implementado de forma assincrona, uma instancia para cada sonograma.
///
/// 1. Define o caminho do espectrograma de acordo com o bool keep_img
///
/// 2. Cria o espectrograma com SoX
///
/// 3. Cria video com filtro de agulha de reproducao usando FFmpeg
///
/// 4. Fim da função, aqui o possivel espectrograma temporario deve ser deletado
pub fn generate_sonogram(audio_path: PathBuf, keep_img: bool) -> bool {
    
    // 1:

    let mut _arquivo_temp = None;

    let img_path = if keep_img {
        audio_path.with_extension("png")
    } else {
        let temp = Builder::new()
            .suffix("png")
            .tempfile()
            .expect("Falha ao criar arquivo temporario");

        let caminho = temp.path().to_path_buf();
        _arquivo_temp = Some(temp); // Ancora o arquivo na função
        caminho
    };

    let video_path = audio_path.with_extension("mp4");

    // 2:

    // sox sample.wav -n remix - spectrogram -x 1920 -Y 1080 -r -o sampleout.png

    let sox_status = Command::new("sox")
        .args(&[
            audio_path.to_str().unwrap(),
            "-n",
            "remix",
            "-", // Downmix estéreo para mono
            "spectrogram",
            "-x",
            "1920", // Largura exata
            "-r",   // Sem bordas
            "-o",
            img_path.to_str().unwrap(),
        ])
        .status()
        .expect("Falha ao invocar o SoX");

    if !sox_status.success() {
        eprintln!("SoX: Erro em {:?}", audio_path);
        return false;
    }

    // 3.

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
        .output()
        .expect("Falha ao executar o ffprobe");

    let duracao_str = str::from_utf8(&ffprobe_output.stdout).unwrap().trim();

    let filtro = format!(
        "[1:v][2:v]overlay=x='(W/{})*t':y=0:shortest=1[video]",
        duracao_str
    );

    let ffmpeg_status = Command::new("ffmpeg")
        .args(&[
            "-i", audio_path.to_str().unwrap(),
            "-loop", "1", "-framerate", "24", "-i", img_path.to_str().unwrap(),
            "-f", "lavfi", "-i", "color=c=black:s=1920x1080",
            "-f", "lavfi", "-i", "color=c=red:s=3x1080",
            "-filter_complex", &filtro,
            "-map", "[video]",
            "-map", "0:a",
            "-c:v", "libx264",
            "-preset", "fast",
            "-pix_fmt", "yuv420p",
            "-c:a", "copy", // Copia o áudio original 1:1
            "-shortest",
            "-y",
            video_path.to_str().unwrap(),
        ])
        .status()
        .expect("Falha ao invocar o ffmpeg");

    // TODO: futuramente remover esses println
    if ffmpeg_status.success() {
        println!("Vídeo finalizado: {:?}", video_path.display());
    } else {
        eprintln!("Erro na renderização do vídeo.");
        return false;
    }

    // 4. 

    true
}