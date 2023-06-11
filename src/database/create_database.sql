CREATE TABLE IF NOT EXISTS products (
    product TEXT NOT NULL,
    category TEXT NOT NULL,
    name TEXT NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS products_product ON products ( product );

CREATE TABLE IF NOT EXISTS tickets (
    ticket TEXT NOT NULL,
    product TEXT NOT NULL,
    quantity REAL NOT NULL,
    sum REAL NOT NULL
);
CREATE INDEX IF NOT EXISTS tickets_ticket ON tickets ( ticket, product );
CREATE INDEX IF NOT EXISTS tickets_product ON tickets ( product );
