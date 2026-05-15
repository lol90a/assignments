use std::env;
use std::path::Path;

fn main() {
    // Register the custom cfg so Rust's check-cfg lint accepts both SQLx modes.
    println!("cargo:rustc-check-cfg=cfg(sqlx_checked)");
    println!("cargo:rerun-if-env-changed=DATABASE_URL");
    println!("cargo:rerun-if-changed=.sqlx");
    println!("cargo:rerun-if-changed=sqlx-data.json");

    let sqlx_check_enabled = env::var_os("CARGO_FEATURE_SQLX_CHECK").is_some();
    // SQLx query macros need either a live database or a prepared offline cache.
    let has_online_database = env::var_os("DATABASE_URL").is_some();
    let has_offline_cache = Path::new(".sqlx").exists() || Path::new("sqlx-data.json").exists();

    if sqlx_check_enabled && (has_online_database || has_offline_cache) {
        // Enable compile-time checked SQL only when the required metadata is
        // available, keeping default local builds dependency-light.
        println!("cargo:rustc-cfg=sqlx_checked");
    } else if sqlx_check_enabled {
        println!(
            "cargo:warning=sqlx-check requested without DATABASE_URL or a prepared .sqlx cache; falling back to runtime-checked queries"
        );
    }
}
