CREATE TABLE tools (
    item_id INTEGER PRIMARY KEY REFERENCES items(id),
    action TEXT NOT NULL,
    durability INTEGER
);
