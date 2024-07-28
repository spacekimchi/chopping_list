CREATE TABLE IF NOT EXISTS recipes_tags (
    id SERIAL PRIMARY KEY,
    tag_id INTEGER NOT NULL REFERENCES tags (id) ON DELETE CASCADE,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipes_tags_tag_id ON recipes_tags(tag_id);
CREATE INDEX idx_recipes_tags_recipe_id ON recipes_tags(recipe_id);
CREATE UNIQUE INDEX idx_recipes_tags_rec_tag ON recipes_tags(tag_id, recipe_id);

CREATE TRIGGER update_recipes_tags_updated_at
BEFORE UPDATE ON recipes_tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

