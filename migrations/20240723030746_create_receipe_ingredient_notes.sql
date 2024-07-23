CREATE TABLE IF NOT EXISTS receipe_ingredient_notes (
    id SERIAL PRIMARY KEY,
    recipe_ingredient_id INTEGER NOT NULL REFERENCES recipe_ingredients (id) ON DELETE CASCADE,
    notes TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_receipe_ingredient_notes_rec_ing_id ON receipe_ingredient_notes(recipe_ingredient_id);

CREATE TRIGGER update_receipe_ingredient_notes_updated_at
BEFORE UPDATE ON receipe_ingredient_notes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

