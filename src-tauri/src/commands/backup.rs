use crate::db::Database;
use crate::db::models::*;
use tauri::State;
use std::path::PathBuf;

#[tauri::command]
pub fn create_backup(db: State<'_, Database>, backup_path: String) -> Result<BackupResult, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let dest_dir = PathBuf::from(&backup_path);
    if !dest_dir.exists() {
        std::fs::create_dir_all(&dest_dir).map_err(|e| format!("No se pudo crear la carpeta: {}", e))?;
    }

    // Generate filename with timestamp
    let now = chrono::Local::now();
    let filename = format!("aynipos_backup_{}.db", now.format("%Y%m%d_%H%M%S"));
    let dest_path = dest_dir.join(&filename);

    // Use SQLite backup API for consistent copy (safe with WAL mode)
    {
        let mut dst = rusqlite::Connection::open(&dest_path)
            .map_err(|e| format!("No se pudo crear archivo de backup: {}", e))?;
        
        let backup = rusqlite::backup::Backup::new(&conn, &mut dst)
            .map_err(|e| format!("Error al iniciar backup: {}", e))?;
        
        backup.run_to_completion(100, std::time::Duration::from_millis(50), None)
            .map_err(|e| format!("Error durante el backup: {}", e))?;
    }

    // Get file size
    let metadata = std::fs::metadata(&dest_path)
        .map_err(|e| format!("Error al leer archivo de backup: {}", e))?;
    let size_bytes = metadata.len();

    // Cleanup old backups — keep only the last 10
    cleanup_old_backups(&dest_dir, 10);

    log::info!("Backup created at {:?} ({} bytes)", dest_path, size_bytes);

    Ok(BackupResult {
        file_path: dest_path.to_string_lossy().to_string(),
        file_name: filename,
        size_bytes,
        created_at: now.format("%Y-%m-%dT%H:%M:%S").to_string(),
    })
}

#[tauri::command]
pub fn get_backup_info(backup_path: String) -> Result<BackupInfo, String> {
    let dest_dir = PathBuf::from(&backup_path);
    
    if backup_path.is_empty() || !dest_dir.exists() {
        return Ok(BackupInfo {
            last_backup: None,
            total_backups: 0,
            total_size_bytes: 0,
        });
    }

    // List backup files
    let mut backups: Vec<(String, u64, String)> = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(&dest_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("aynipos_backup_") && name.ends_with(".db") {
                if let Ok(meta) = entry.metadata() {
                    let modified = meta.modified()
                        .map(|t| {
                            let dt: chrono::DateTime<chrono::Local> = t.into();
                            dt.format("%Y-%m-%dT%H:%M:%S").to_string()
                        })
                        .unwrap_or_default();
                    backups.push((name, meta.len(), modified));
                }
            }
        }
    }

    backups.sort_by(|a, b| b.2.cmp(&a.2)); // Sort by date descending

    let total_backups = backups.len() as u32;
    let total_size_bytes: u64 = backups.iter().map(|b| b.1).sum();
    
    let last_backup = backups.first().map(|(name, size, date)| BackupResult {
        file_path: dest_dir.join(name).to_string_lossy().to_string(),
        file_name: name.clone(),
        size_bytes: *size,
        created_at: date.clone(),
    });

    Ok(BackupInfo {
        last_backup,
        total_backups,
        total_size_bytes,
    })
}

fn cleanup_old_backups(dir: &PathBuf, keep: usize) {
    let mut backups: Vec<(PathBuf, std::time::SystemTime)> = Vec::new();
    
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("aynipos_backup_") && name.ends_with(".db") {
                if let Ok(meta) = entry.metadata() {
                    if let Ok(modified) = meta.modified() {
                        backups.push((entry.path(), modified));
                    }
                }
            }
        }
    }

    backups.sort_by(|a, b| b.1.cmp(&a.1)); // Newest first

    // Remove files beyond the keep limit
    for (path, _) in backups.iter().skip(keep) {
        if let Err(e) = std::fs::remove_file(path) {
            log::warn!("No se pudo eliminar backup antiguo {:?}: {}", path, e);
        } else {
            log::info!("Backup antiguo eliminado: {:?}", path);
        }
    }
}
