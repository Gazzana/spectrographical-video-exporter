use std::process::Command;
use std::path::PathBuf;



/// Deve ser implementado de forma assincrona, uma instancia para cada sonograma.
/// 
/// 1. Define o caminho do espectrograma de acordo com o bool keep_img
/// 
/// 2. Cria o espectrograma com SoX
/// 
/// 3. Cria video com filtro de agulha de reproducao usando FFmpeg
/// 
/// 4. Fim da função, aqui o possivel espectrograma temporario deve ser deletado
pub fn generate_sonogram(audio_path: PathBuf, sonograma_path: PathBuf) {
    //                      TODO: ARRUMAR IMEDIATAMENTE

    let audio_path_str = &audio_path.display().to_string();
    let sonograma_path_str = &sonograma_path.display().to_string();

    let output = audio_path.display().to_string() + ".mp4";

    let ffprobe_output = Command::new("ffprobe")
        .args(&[
            "-v",
            "error",
            "-show_entries",
            "format=duration",
            "-of",
            "default=noprint_wrappers=1:nokey=1",
            audio_path_str,
        ])
        .output()
        .expect("Falha ao executar o ffprobe");

    let duracao = str::from_utf8(&ffprobe_output.stdout).unwrap().trim();

    // montar o filtro complexo com a duração real
    // ex: "[1:v][2:v]overlay=x='(W/125.4)*t':y=0:shortest=1[video]"
    let filtro = format!(
        "[1:v][2:v]overlay=x='(W/{})*t':y=0:shortest=1[video]",
        duracao
    );

    // executar o ffmpeg
    let status = Command::new("ffmpeg")
        .args(&[
            "-i",
            audio_path_str,
            "-loop",
            "1",
            "-framerate",
            "24",
            "-i",
            sonograma_path_str,
            "-f",
            "lavfi",
            "-i",
            "color=c=red:s=3x1080", // agulha vermelha de 3px
            "-filter_complex",
            &filtro,
            "-map",
            "[video]",
            "-map",
            "0:a",
            "-c:v",
            "libx264",
            "-preset",
            "fast", // Mude para "ultrafast" se quiser renderizar em segundos
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "copy",
            "-shortest",
            "-y", // Sobrescreve o arquivo de saída se existir
            &output,
        ])
        .status()
        .expect("Falha ao executar o ffmpeg");
    if status.success() {
        println!("Vídeo renderizado com sucesso!");
    } else {
        eprintln!("Erro na renderização.");
    }
}