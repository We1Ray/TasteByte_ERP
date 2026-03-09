#!/usr/bin/env bash
# ============================================================
# TasteByte ERP — Database Restore Script
#
# Usage:
#   ./scripts/restore.sh <backup_file>
#
# The backup file can be:
#   - A gzipped SQL dump (.sql.gz)
#   - A plain SQL dump (.sql)
#
# Environment variables (or defaults):
#   DATABASE_URL    — full connection string
#   PGHOST          — PostgreSQL host       (default: localhost)
#   PGPORT          — PostgreSQL port       (default: 5432)
#   PGUSER          — PostgreSQL user       (default: postgres)
#   PGPASSWORD      — PostgreSQL password   (default: postgres)
#   PGDATABASE      — PostgreSQL database   (default: TastyByte)
# ============================================================

set -euo pipefail

# ── Usage ───────────────────────────────────────────────────
usage() {
    echo "Usage: $0 <backup_file>"
    echo ""
    echo "Restore a TasteByte ERP database from a backup file."
    echo ""
    echo "Arguments:"
    echo "  backup_file   Path to a .sql or .sql.gz backup file"
    echo ""
    echo "Environment variables:"
    echo "  DATABASE_URL   Full connection string (overrides individual vars)"
    echo "  PGHOST         PostgreSQL host       (default: localhost)"
    echo "  PGPORT         PostgreSQL port       (default: 5432)"
    echo "  PGUSER         PostgreSQL user       (default: postgres)"
    echo "  PGPASSWORD     PostgreSQL password   (default: postgres)"
    echo "  PGDATABASE     PostgreSQL database   (default: TastyByte)"
    exit 1
}

# ── Validate arguments ─────────────────────────────────────
if [ $# -lt 1 ]; then
    echo "Error: No backup file specified."
    echo ""
    usage
fi

BACKUP_FILE="$1"

if [ ! -f "${BACKUP_FILE}" ]; then
    echo "Error: Backup file not found: ${BACKUP_FILE}"
    exit 1
fi

# ── Parse connection parameters ────────────────────────────
if [ -n "${DATABASE_URL:-}" ]; then
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

log() {
    echo "[$(date +%Y-%m-%dT%H:%M:%S%z)] $1"
}

# ── Confirm with user ──────────────────────────────────────
echo "============================================================"
echo "  TasteByte ERP — Database Restore"
echo "============================================================"
echo ""
echo "  Database:    ${PGDATABASE}"
echo "  Host:        ${PGHOST}:${PGPORT}"
echo "  User:        ${PGUSER}"
echo "  Backup file: ${BACKUP_FILE}"
echo ""
echo "WARNING: This will DROP and RECREATE the '${PGDATABASE}' database."
echo "         All existing data will be permanently lost."
echo ""
printf "Continue? [y/N] "
read -r confirm
if [ "${confirm}" != "y" ] && [ "${confirm}" != "Y" ]; then
    echo "Restore cancelled."
    exit 0
fi

# ── Drop and recreate database ─────────────────────────────
log "Terminating active connections to '${PGDATABASE}'..."
psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d postgres -c \
    "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '${PGDATABASE}' AND pid <> pg_backend_pid();" \
    > /dev/null 2>&1 || true

log "Dropping database '${PGDATABASE}'..."
dropdb -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" --if-exists "${PGDATABASE}"

log "Creating database '${PGDATABASE}'..."
createdb -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" "${PGDATABASE}"

# ── Restore from backup ───────────────────────────────────
log "Restoring from '${BACKUP_FILE}'..."

case "${BACKUP_FILE}" in
    *.sql.gz)
        gunzip -c "${BACKUP_FILE}" | psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" --quiet
        ;;
    *.sql)
        psql -h "${PGHOST}" -p "${PGPORT}" -U "${PGUSER}" -d "${PGDATABASE}" --quiet < "${BACKUP_FILE}"
        ;;
    *)
        echo "Error: Unsupported file format. Use .sql or .sql.gz"
        exit 1
        ;;
esac

log "Restore complete. Database '${PGDATABASE}' has been restored from '${BACKUP_FILE}'."
