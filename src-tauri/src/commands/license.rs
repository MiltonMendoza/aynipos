use tauri::State;
use crate::db::Database;
use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};
use serde::Serialize;

type HmacSha256 = Hmac<Sha256>;

// ─── Secret key for HMAC signing ───────────────────────
// This key is used to sign and verify license keys.
// It must match the key used in the license-gen CLI tool.
const LICENSE_SECRET: &[u8] = b"AyniPOS-2026-LicSec-K3y!@#$MilM3n";

const TRIAL_DAYS: i64 = 20;

// ─── Response types ────────────────────────────────────

#[derive(Serialize)]
pub struct LicenseStatus {
    pub status: String,           // "trial" | "active" | "expired"
    pub machine_id: String,
    pub days_remaining: Option<i64>,
    pub license_type: Option<String>,  // "perpetual" | "subscription"
    pub expiry_date: Option<String>,
}

// ─── Helper functions ──────────────────────────────────

/// Get a unique machine fingerprint as XXXX-XXXX format
fn get_machine_fingerprint() -> String {
    let uid = machine_uid::get().unwrap_or_else(|_| "unknown-machine".to_string());
    let mut hasher = Sha256::new();
    hasher.update(uid.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    // Take first 8 hex chars, format as XXXX-XXXX
    format!("{}-{}", &hash[..4], &hash[4..8]).to_uppercase()
}

/// Verify a license key against a machine ID
fn verify_license_key(key: &str, expected_machine_id: &str) -> Result<(String, Option<String>), String> {
    // License format: BASE64(payload).HMAC_HEX
    let parts: Vec<&str> = key.trim().split('.').collect();
    if parts.len() != 2 {
        return Err("Formato de licencia inválido".to_string());
    }

    let payload_b64 = parts[0];
    let signature_hex = parts[1];

    // Verify HMAC signature
    let mut mac = HmacSha256::new_from_slice(LICENSE_SECRET)
        .map_err(|e| format!("Error de clave: {}", e))?;
    mac.update(payload_b64.as_bytes());

    let signature_bytes = hex::decode(signature_hex)
        .map_err(|_| "Firma de licencia inválida".to_string())?;

    mac.verify_slice(&signature_bytes)
        .map_err(|_| "Licencia no válida: firma incorrecta".to_string())?;

    // Decode payload
    use base64::Engine;
    let payload_bytes = base64::engine::general_purpose::STANDARD
        .decode(payload_b64)
        .map_err(|_| "Licencia corrupta".to_string())?;
    let payload = String::from_utf8(payload_bytes)
        .map_err(|_| "Licencia corrupta".to_string())?;

    // Parse payload: machine_id|type|expiry
    let fields: Vec<&str> = payload.split('|').collect();
    if fields.len() != 3 {
        return Err("Estructura de licencia inválida".to_string());
    }

    let license_machine_id = fields[0];
    let license_type = fields[1];
    let license_expiry = fields[2];

    // Verify machine ID matches
    if license_machine_id != expected_machine_id {
        return Err("Esta licencia no corresponde a esta máquina".to_string());
    }

    // Check expiry for subscription licenses
    if license_type == "subscription" && license_expiry != "none" {
        let expiry = chrono::NaiveDate::parse_from_str(license_expiry, "%Y-%m-%d")
            .map_err(|_| "Fecha de expiración inválida".to_string())?;
        let today = (chrono::Utc::now() - chrono::Duration::hours(4)).date_naive();
        if today > expiry {
            return Err("Esta licencia ha expirado".to_string());
        }
    }

    let expiry = if license_expiry == "none" {
        None
    } else {
        Some(license_expiry.to_string())
    };

    Ok((license_type.to_string(), expiry))
}

/// Ensure first_run_date exists in settings
fn ensure_first_run_date(conn: &rusqlite::Connection) -> Result<String, String> {
    let existing: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'first_run_date'",
            [],
            |row| row.get(0),
        )
        .ok();

    if let Some(date) = existing {
        Ok(date)
    } else {
        let now = (chrono::Utc::now() - chrono::Duration::hours(4)).format("%Y-%m-%d").to_string();
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('first_run_date', ?1)",
            [&now],
        )
        .map_err(|e| e.to_string())?;
        Ok(now)
    }
}

// ─── Tauri Commands ────────────────────────────────────

#[tauri::command]
pub fn get_machine_id() -> Result<String, String> {
    Ok(get_machine_fingerprint())
}

#[tauri::command]
pub fn get_license_status(db: State<Database>) -> Result<LicenseStatus, String> {
    let conn = db.conn.lock().unwrap();
    let machine_id = get_machine_fingerprint();

    // Check if there's a saved license key
    let saved_key: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'license_key'",
            [],
            |row| row.get(0),
        )
        .ok();

    if let Some(key) = saved_key {
        if !key.is_empty() {
            // Try to verify the saved license
            match verify_license_key(&key, &machine_id) {
                Ok((license_type, expiry_date)) => {
                    return Ok(LicenseStatus {
                        status: "active".to_string(),
                        machine_id,
                        days_remaining: None,
                        license_type: Some(license_type),
                        expiry_date,
                    });
                }
                Err(_) => {
                    // License invalid (maybe machine changed), clear it
                    let _ = conn.execute(
                        "DELETE FROM settings WHERE key = 'license_key'",
                        [],
                    );
                }
            }
        }
    }

    // No valid license — check trial
    let first_run = ensure_first_run_date(&conn)?;
    let first_run_date = chrono::NaiveDate::parse_from_str(&first_run, "%Y-%m-%d")
        .map_err(|e| format!("Error de fecha: {}", e))?;
    let today = (chrono::Utc::now() - chrono::Duration::hours(4)).date_naive();
    let days_elapsed = (today - first_run_date).num_days();
    let days_remaining = TRIAL_DAYS - days_elapsed;

    if days_remaining > 0 {
        Ok(LicenseStatus {
            status: "trial".to_string(),
            machine_id,
            days_remaining: Some(days_remaining),
            license_type: None,
            expiry_date: None,
        })
    } else {
        Ok(LicenseStatus {
            status: "expired".to_string(),
            machine_id,
            days_remaining: Some(0),
            license_type: None,
            expiry_date: None,
        })
    }
}

#[tauri::command]
pub fn activate_license(db: State<Database>, key: String) -> Result<LicenseStatus, String> {
    let conn = db.conn.lock().unwrap();
    let machine_id = get_machine_fingerprint();

    // Verify the license key
    let (license_type, expiry_date) = verify_license_key(&key, &machine_id)?;

    // Save the license key
    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('license_key', ?1)",
        [&key],
    )
    .map_err(|e| e.to_string())?;

    Ok(LicenseStatus {
        status: "active".to_string(),
        machine_id,
        days_remaining: None,
        license_type: Some(license_type),
        expiry_date,
    })
}

#[tauri::command]
pub fn deactivate_license(db: State<Database>) -> Result<(), String> {
    let conn = db.conn.lock().unwrap();
    conn.execute(
        "DELETE FROM settings WHERE key = 'license_key'",
        [],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
