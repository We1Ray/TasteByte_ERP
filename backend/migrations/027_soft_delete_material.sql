-- mm_materials already has is_active column, no schema change needed.
-- This migration adds a check to ensure is_active defaults properly.
ALTER TABLE mm_materials ALTER COLUMN is_active SET DEFAULT true;
