-- Your SQL goes here
-- Create crops table
CREATE TABLE crops (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    yield_item INT NOT NULL,
    sell_price INT NOT NULL
);

-- Create crop_growth_stages table
CREATE TABLE crop_growth_stages (
    id SERIAL PRIMARY KEY,
    crop_id INT NOT NULL REFERENCES crops(id) ON DELETE CASCADE,
    stage_index INT NOT NULL,
    stage_name TEXT NOT NULL,
    duration_seconds INT NOT NULL,
    sprite_url TEXT NOT NULL
);

-- Optional: index for faster lookups
CREATE INDEX idx_crop_growth_stages_crop_id ON crop_growth_stages (crop_id);
