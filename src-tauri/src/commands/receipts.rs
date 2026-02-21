use std::fs;
use std::env;

/// Save receipt HTML to a temp file and return the file path.
/// The frontend will open it in the system browser using the opener plugin.
#[tauri::command]
pub fn save_receipt_html(html: String) -> Result<String, String> {
    let temp_dir = env::temp_dir();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("aynipos_recibo_{}.html", timestamp);
    let filepath = temp_dir.join(&filename);

    fs::write(&filepath, html.as_bytes())
        .map_err(|e| format!("Error al guardar recibo: {}", e))?;

    Ok(filepath.to_string_lossy().to_string())
}
