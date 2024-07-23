CREATE TABLE IF NOT EXISTS ingredients (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_ingredients_name ON ingredients(name);

CREATE TRIGGER update_ingredients_updated_at
BEFORE UPDATE ON ingredients
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

