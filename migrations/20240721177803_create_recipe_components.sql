CREATE TABLE IF NOT EXISTS recipe_components (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    is_optional BOOL NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_components_recipe_id ON recipe_components(recipe_id);

CREATE TRIGGER update_recipe_components_updated_at
BEFORE UPDATE ON recipe_components
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

