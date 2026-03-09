-- CO Auto-Posting: Add source tracking columns and seed default cost centers

-- Add source_module and reference_id columns for tracking auto-posted allocations
ALTER TABLE co_cost_allocations
    ADD COLUMN IF NOT EXISTS source_module VARCHAR(20),
    ADD COLUMN IF NOT EXISTS reference_id UUID;

-- Create index for efficient lookups by source
CREATE INDEX IF NOT EXISTS idx_co_cost_allocations_source
    ON co_cost_allocations (source_module, reference_id)
    WHERE source_module IS NOT NULL;

-- Seed default cost centers used by auto-posting if they don't already exist
INSERT INTO co_cost_centers (code, name, description)
VALUES
    ('CC-GENERAL', 'General Overhead', 'Default cost center for general overhead and auto-posting source'),
    ('CC-PROCUREMENT', 'Procurement', 'Cost center for procurement and purchasing costs'),
    ('CC-PRODUCTION', 'Production', 'Cost center for production and manufacturing costs')
ON CONFLICT (code) DO NOTHING;
