CREATE TABLE IF NOT EXISTS recipe_instructions (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    order_idx INTEGER NOT NULL,
    title TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_instructions_recipe_id ON recipe_instructions(recipe_id);
CREATE UNIQUE INDEX idx_recipe_instructions_order_idx ON recipe_instructions(recipe_id, order_idx);

CREATE TRIGGER update_recipe_instructions_updated_at
BEFORE UPDATE ON recipe_instructions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

