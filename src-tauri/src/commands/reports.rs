use std::fs;
use std::env;

/// Save CSV content to the specified file path.
#[tauri::command]
pub fn save_report_csv(content: String, file_path: String) -> Result<(), String> {
    fs::write(&file_path, content.as_bytes())
        .map_err(|e| format!("Error al guardar reporte CSV: {}", e))?;
    Ok(())
}

/// Save report HTML to a temp file and return the file path.
/// The frontend will open it in the system browser for printing as PDF.
#[tauri::command]
pub fn save_report_html(html: String) -> Result<String, String> {
    let temp_dir = env::temp_dir();
    let timestamp = chrono::Local::now().format("%Y%m%d_%H%M%S");
    let filename = format!("aynipos_reporte_{}.html", timestamp);
    let filepath = temp_dir.join(&filename);

    fs::write(&filepath, html.as_bytes())
        .map_err(|e| format!("Error al guardar reporte: {}", e))?;

    Ok(filepath.to_string_lossy().to_string())
}
