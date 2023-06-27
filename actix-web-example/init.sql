CREATE TABLE IF NOT EXISTS products (
    product_name TEXT,
    title TEXT NOT NULL,
    gtin Text,
    PRIMARY KEY (product_name)
);
INSERT INTO products VALUES ('testproduct', 'Our first Testproduct','0811571013579') ON CONFLICT DO NOTHING;