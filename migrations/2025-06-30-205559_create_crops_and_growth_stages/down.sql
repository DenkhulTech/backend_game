-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_crop_growth_stages_crop_id;
DROP TABLE IF EXISTS crop_growth_stages;
DROP TABLE IF EXISTS crops;
