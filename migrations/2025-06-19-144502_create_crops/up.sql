-- Your SQL goes here
CREATE TABLE crops (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    grow_time INTEGER NOT NULL,
    yield_item INTEGER NOT NULL,
    sell_price INTEGER NOT NULL
);
