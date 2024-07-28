CREATE TABLE IF NOT EXISTS ingredients_tags (
    id SERIAL PRIMARY KEY,
    tag_id INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    ingredient_id INTEGER NOT NULL REFERENCES ingredients (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ingredients_tags_tag_id ON ingredients_tags(tag_id);
CREATE INDEX idx_ingredients_tags_ingredient_id ON ingredients_tags(ingredient_id);
CREATE UNIQUE INDEX idx_ingredients_tags_ing_tag ON ingredients_tags(tag_id, ingredient_id);

CREATE TRIGGER update_ingredients_tags_updated_at
BEFORE UPDATE ON ingredients_tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

