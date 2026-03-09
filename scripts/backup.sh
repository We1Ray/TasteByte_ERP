#!/usr/bin/env bash
# ============================================================
# TasteByte ERP — Database Backup Script
#
# Usage:
#   ./scripts/backup.sh
#
# Environment variables (or defaults):
#   DATABASE_URL    — full connection string
#   PGHOST          — PostgreSQL host       (default: localhost)
#   PGPORT          — PostgreSQL port       (default: 5432)
#   PGUSER          — PostgreSQL user       (default: postgres)
#   PGPASSWORD      — PostgreSQL password   (default: postgres)
#   PGDATABASE      — PostgreSQL database   (default: TastyByte)
#   BACKUP_DIR      — backup directory      (default: ./backups)
#   BACKUP_RETAIN   — days to retain        (default: 7)
# ============================================================

set -euo pipefail

# ── Configuration ────────────────────────────────────────────
BACKUP_DIR="${BACKUP_DIR:-./backups}"
BACKUP_RETAIN="${BACKUP_RETAIN:-7}"
TIMESTAMP="$(date +%Y%m%d_%H%M%S)"

# Parse DATABASE_URL if provided, otherwise use individual vars
if [ -n "${DATABASE_URL:-}" ]; then
    # Extract components from postgres://user:pass@host:port/dbname
    # Remove the protocol prefix
    DB_STRING="${DATABASE_URL#postgres://}"
    DB_STRING="${DB_STRING#postgresql://}"

    PGUSER="${DB_STRING%%:*}"
    DB_STRING="${DB_STRING#*:}"

    PGPASSWORD="${DB_STRING%%@*}"
    DB_STRING="${DB_STRING#*@}"

    PGHOST="${DB_STRING%%:*}"
    DB_STRING="${DB_STRING#*:}"

    PGPORT="${DB_STRING%%/*}"
    PGDATABASE="${DB_STRING#*/}"
else
    PGHOST="${PGHOST:-localhost}"
    PGPORT="${PGPORT:-5432}"
    PGUSER="${PGUSER:-postgres}"
    PGPASSWORD="${PGPASSWORD:-postgres}"
    PGDATABASE="${PGDATABASE:-TastyByte}"
fi

export PGPASSWORD

BACKUP_FILE="${BACKUP_DIR}/${PGDATABASE}_${TIMESTAMP}.sql.gz"

# ── Ensure backup directory exists ───────────────────────────
mkdir -p "${BACKUP_DIR}"

# ── Run backup ───────────────────────────────────────────────
echo "[$(date --iso-8601=seconds 2>/dev/null || date +%Y-%m-%dT%H:%M:%S%z)] Starting backup of '${PGDATABASE}' on ${PGHOST}:${PGPORT}"

pg_dump \
    -h "${PGHOST}" \
    -p "${PGPORT}" \
    -U "${PGUSER}" \
    -d "${PGDATABASE}" \
    --format=plain \
    --no-owner \
    --no-privileges \
    --verbose 2>&1 \
    | gzip > "${BACKUP_FILE}"

FILESIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
echo "[$(date --iso-8601=seconds 2>/dev/null || date +%Y-%m-%dT%H:%M:%S%z)] Backup complete: ${BACKUP_FILE} (${FILESIZE})"

# ── Rotate old backups ───────────────────────────────────────
echo "[$(date --iso-8601=seconds 2>/dev/null || date +%Y-%m-%dT%H:%M:%S%z)] Rotating backups — keeping last ${BACKUP_RETAIN} days"

DELETED=0
find "${BACKUP_DIR}" \
    -name "${PGDATABASE}_*.sql.gz" \
    -type f \
    -mtime "+${BACKUP_RETAIN}" \
    -print \
    -delete 2>/dev/null | while read -r file; do
    echo "  Deleted: ${file}"
    DELETED=$((DELETED + 1))
done

echo "[$(date --iso-8601=seconds 2>/dev/null || date +%Y-%m-%dT%H:%M:%S%z)] Backup rotation complete"

# ── List remaining backups ───────────────────────────────────
echo ""
echo "Current backups in ${BACKUP_DIR}:"
ls -lh "${BACKUP_DIR}"/${PGDATABASE}_*.sql.gz 2>/dev/null || echo "  (none)"
