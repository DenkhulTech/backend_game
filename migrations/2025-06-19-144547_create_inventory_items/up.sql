-- Your SQL goes here
CREATE TABLE inventory_items (
    id SERIAL PRIMARY KEY,
    user_id INTEGER REFERENCES users(id),
    item_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL
);
