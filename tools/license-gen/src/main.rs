use clap::Parser;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use base64::Engine;

type HmacSha256 = Hmac<Sha256>;

// ⚠️ This MUST match the secret in src-tauri/src/commands/license.rs
const LICENSE_SECRET: &[u8] = b"AyniPOS-2026-LicSec-K3y!@#$MilM3n";

#[derive(Parser)]
#[command(name = "license-gen")]
#[command(about = "Genera claves de licencia para AyniPOS")]
struct Cli {
    /// Código de máquina del cliente (formato XXXX-XXXX)
    #[arg(short, long)]
    machine_id: String,

    /// Tipo de licencia: perpetual o subscription
    #[arg(short = 't', long, default_value = "perpetual")]
    license_type: String,

    /// Fecha de expiración (YYYY-MM-DD). Solo para subscriptions.
    #[arg(short, long, default_value = "none")]
    expiry: String,
}

fn main() {
    let cli = Cli::parse();

    // Validate machine ID format
    let machine_id = cli.machine_id.trim().to_uppercase();
    if machine_id.len() != 9 || !machine_id.contains('-') {
        eprintln!("❌ Error: El código de máquina debe tener formato XXXX-XXXX");
        eprintln!("   Recibido: \"{}\"", machine_id);
        std::process::exit(1);
    }

    // Validate license type
    let license_type = cli.license_type.to_lowercase();
    if license_type != "perpetual" && license_type != "subscription" {
        eprintln!("❌ Error: El tipo de licencia debe ser 'perpetual' o 'subscription'");
        std::process::exit(1);
    }

    // Validate expiry
    let expiry = if license_type == "subscription" && cli.expiry == "none" {
        eprintln!("❌ Error: Las licencias de suscripción requieren una fecha de expiración (--expiry YYYY-MM-DD)");
        std::process::exit(1);
    } else {
        cli.expiry.clone()
    };

    // Build payload: machine_id|type|expiry
    let payload = format!("{}|{}|{}", machine_id, license_type, expiry);
    let payload_b64 = base64::engine::general_purpose::STANDARD.encode(payload.as_bytes());

    // Sign with HMAC-SHA256
    let mut mac = HmacSha256::new_from_slice(LICENSE_SECRET)
        .expect("HMAC can take key of any size");
    mac.update(payload_b64.as_bytes());
    let signature = hex::encode(mac.finalize().into_bytes());

    // Build license key
    let license_key = format!("{}.{}", payload_b64, signature);

    println!();
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║           🔑 AyniPOS — Licencia Generada           ║");
    println!("╠══════════════════════════════════════════════════════╣");
    println!("║ Máquina:  {}                              ║", machine_id);
    println!("║ Tipo:     {:<42} ║", license_type);
    println!("║ Expira:   {:<42} ║", expiry);
    println!("╠══════════════════════════════════════════════════════╣");
    println!("║ Clave de licencia:                                  ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!();
    println!("{}", license_key);
    println!();
    println!("📋 Copie la clave de arriba y envíela al cliente.");
}
