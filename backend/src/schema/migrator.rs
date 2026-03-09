use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::path::Path;
use tracing::{info, warn};

pub async fn run_migrations(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    // Create _schema_versions table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _schema_versions (
            version INT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            checksum VARCHAR(64) NOT NULL,
            applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    let migrations_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("migrations");
    if !migrations_dir.exists() {
        warn!("Migrations directory not found: {:?}", migrations_dir);
        return Ok(());
    }

    let mut entries: Vec<_> = std::fs::read_dir(&migrations_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "sql"))
        .collect();

    entries.sort_by_key(|e| e.file_name());

    let dry_run = std::env::var("DRY_RUN").unwrap_or_default() == "true";

    for entry in entries {
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();

        // Extract version number from filename (e.g., "001_foundation.sql" -> 1)
        let version: i32 = name
            .split('_')
            .next()
            .and_then(|v| v.parse().ok())
            .ok_or_else(|| format!("Invalid migration filename: {}", name))?;

        let sql = std::fs::read_to_string(entry.path())
            .map_err(|e| format!("Failed to read migration {}: {}", name, e))?;

        let mut hasher = Sha256::new();
        hasher.update(sql.as_bytes());
        let checksum = hex::encode(hasher.finalize());

        // Check if already applied
        let applied: Option<(String,)> =
            sqlx::query_as("SELECT checksum FROM _schema_versions WHERE version = $1")
                .bind(version)
                .fetch_optional(pool)
                .await?;

        if let Some((existing_checksum,)) = applied {
            if existing_checksum != checksum {
                warn!(
                    "Migration {} has changed checksum! Expected: {}, Found: {}",
                    name, existing_checksum, checksum
                );
            }
            continue;
        }

        if dry_run {
            info!("[DRY RUN] Would apply migration: {}", name);
            continue;
        }

        info!("Applying migration: {}", name);

        // Execute migration in a transaction
        let mut tx = pool.begin().await?;

        sqlx::raw_sql(&sql)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to execute migration {}: {}", name, e))?;

        sqlx::query("INSERT INTO _schema_versions (version, name, checksum) VALUES ($1, $2, $3)")
            .bind(version)
            .bind(name.as_ref())
            .bind(&checksum)
            .execute(&mut *tx)
            .await
            .map_err(|e| format!("Failed to record migration {}: {}", name, e))?;

        tx.commit().await?;

        info!("Applied migration: {}", name);
    }

    info!("All migrations applied successfully");
    Ok(())
}
