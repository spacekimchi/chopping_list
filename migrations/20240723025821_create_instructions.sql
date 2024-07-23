CREATE TABLE IF NOT EXISTS instructions (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    position INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_instructions_recipe_id ON instructions(recipe_id);

CREATE TRIGGER update_instructions_updated_at
BEFORE UPDATE ON instructions
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

