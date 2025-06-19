-- Your SQL goes here
CREATE TABLE fields (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    tile_x INTEGER NOT NULL,
    tile_y INTEGER NOT NULL,
    is_tilled BOOLEAN NOT NULL DEFAULT false,
    is_watered BOOLEAN NOT NULL DEFAULT false,
    crop_id INTEGER,
    plant_time TIMESTAMP,
    ready_time TIMESTAMP,
    status TEXT NOT NULL
);
