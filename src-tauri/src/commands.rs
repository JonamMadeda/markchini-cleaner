use std::fs;
use tauri::AppHandle;
use tauri_plugin_dialog::DialogExt;
use base64::Engine as _;

#[derive(serde::Serialize)]
pub struct FileResult {
    pub path: String,
    pub content: String,
}

#[tauri::command]
pub fn pick_markdown_file(app: AppHandle) -> Result<Option<FileResult>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown", "mdown", "mdwn"])
        .add_filter("All Files", &["*"])
        .set_title("Open Markdown File")
        .blocking_pick_file();

    match file {
        Some(path_buf) => {
            let path = path_buf
                .into_path()
                .map_err(|_| "Failed to resolve file path".to_string())?;
            let path_str = path.to_string_lossy().to_string();
            let content = fs::read_to_string(&path_str).map_err(|e| e.to_string())?;
            Ok(Some(FileResult {
                path: path_str,
                content,
            }))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub fn pick_markdown_save_path(app: AppHandle) -> Result<Option<String>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Markdown", &["md", "markdown"])
        .set_title("Save Markdown As")
        .blocking_save_file();

    match file {
        Some(path_buf) => {
            let path = path_buf
                .into_path()
                .map_err(|_| "Failed to resolve save path".to_string())?;
            let mut path_str = path.to_string_lossy().to_string();
            if !path_str.ends_with(".md") && !path_str.ends_with(".markdown") {
                path_str.push_str(".md");
            }
            Ok(Some(path_str))
        }
        None => Ok(None),
    }
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, String> {
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn write_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn show_error(app: AppHandle, title: String, message: String) -> Result<(), String> {
    app.dialog()
        .message(&message)
        .title(&title)
        .kind(tauri_plugin_dialog::MessageDialogKind::Error)
        .blocking_show();
    Ok(())
}

#[tauri::command]
#[allow(unused_variables)]
pub fn compile_pdf(
    app: AppHandle,
    markdown: String,
    font: String,
    font_size: String,
    margin_t: String,
    margin_r: String,
    margin_b: String,
    margin_l: String,
) -> Result<(), String> {
    let path = app
        .dialog()
        .file()
        .add_filter("PDF", &["pdf"])
        .set_title("Save PDF As")
        .blocking_save_file();

    let save_path = match path {
        Some(p) => {
            let mut s = p
                .into_path()
                .map_err(|_| "Failed to resolve save path".to_string())?
                .to_string_lossy()
                .to_string();
            if !s.ends_with(".pdf") {
                s.push_str(".pdf");
            }
            s
        }
        None => return Ok(()),
    };

    let pdf_bytes = crate::html_renderer::render(&markdown)?;
    fs::write(&save_path, pdf_bytes).map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct ImageResult {
    pub mime: String,
    pub data: String,
}

#[tauri::command]
pub fn pick_image(app: AppHandle) -> Result<Option<ImageResult>, String> {
    let file = app
        .dialog()
        .file()
        .add_filter("Images", &["png", "jpg", "jpeg", "gif", "webp", "bmp", "svg"])
        .add_filter("All Files", &["*"])
        .set_title("Insert Image")
        .blocking_pick_file();

    match file {
        Some(path_buf) => {
            let path = path_buf
                .into_path()
                .map_err(|_| "Failed to resolve file path".to_string())?;
            let path_str = path.to_string_lossy().to_string();
            let mime = match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
                "png" => "image/png",
                "jpg" | "jpeg" => "image/jpeg",
                "gif" => "image/gif",
                "webp" => "image/webp",
                "bmp" => "image/bmp",
                "svg" => "image/svg+xml",
                _ => "image/png",
            }
            .to_string();
            let bytes = fs::read(&path_str).map_err(|e| e.to_string())?;
            let data = base64::engine::general_purpose::STANDARD.encode(&bytes);
            Ok(Some(ImageResult { mime, data }))
        }
        None => Ok(None),
    }
}
