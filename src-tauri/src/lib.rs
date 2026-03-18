use std::fs;
use std::path::Path;
use libheif_rs::{ColorSpace, HeifContext, LibHeif, RgbChroma};
use image::{ImageBuffer, Rgb};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct ConvertResult {
    pub file: String,
    pub success: bool,
    pub output_path: String,
    pub error: Option<String>,
}

/// Convert a single HEIC file to JPG.
/// Returns the output path on success.
fn convert_one(input_path: &str, output_dir: &str, quality: u8) -> Result<String, String> {
    let input = Path::new(input_path);
    let stem = input
        .file_stem()
        .ok_or("ファイル名が無効です")?
        .to_string_lossy();

    // Create output directory
    fs::create_dir_all(output_dir)
        .map_err(|e| format!("出力フォルダの作成に失敗: {e}"))?;

    let output_path = format!("{}/{}.jpg", output_dir.trim_end_matches('/'), stem);

    // Decode HEIC
    let lib = LibHeif::new();
    let ctx = HeifContext::read_from_file(input_path)
        .map_err(|e| format!("HEICファイルの読み込みに失敗: {e}"))?;

    let handle = ctx
        .primary_image_handle()
        .map_err(|e| format!("画像ハンドルの取得に失敗: {e}"))?;

    let width = handle.width();
    let height = handle.height();

    let image = lib
        .decode(&handle, ColorSpace::Rgb(RgbChroma::Rgb), None)
        .map_err(|e| format!("デコードに失敗: {e}"))?;

    let plane = image
        .planes()
        .interleaved
        .ok_or("RGBプレーンが取得できません")?;

    let stride = plane.stride;
    let data = plane.data;

    // Build image buffer (handle stride padding)
    let mut pixels: Vec<u8> = Vec::with_capacity((width * height * 3) as usize);
    for row in 0..height as usize {
        let start = row * stride;
        let end = start + (width as usize) * 3;
        pixels.extend_from_slice(&data[start..end]);
    }

    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, pixels)
        .ok_or("画像バッファの構築に失敗")?;

    // Encode to JPEG
    let mut out_file = fs::File::create(&output_path)
        .map_err(|e| format!("出力ファイルの作成に失敗: {e}"))?;

    let mut encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out_file, quality);
    encoder
        .encode_image(&img)
        .map_err(|e| format!("JPEGエンコードに失敗: {e}"))?;

    Ok(output_path)
}

#[tauri::command]
fn open_folder(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    std::process::Command::new("open")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    std::process::Command::new("explorer")
        .arg(&path)
        .spawn()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
fn convert_heic(input_path: String, output_dir: String, quality: u8) -> ConvertResult {
    let name = Path::new(&input_path)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| input_path.clone());

    match convert_one(&input_path, &output_dir, quality) {
        Ok(out) => ConvertResult {
            file: name,
            success: true,
            output_path: out,
            error: None,
        },
        Err(e) => ConvertResult {
            file: name,
            success: false,
            output_path: String::new(),
            error: Some(e),
        },
    }
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![convert_heic, open_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
