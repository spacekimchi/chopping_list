CREATE TABLE IF NOT EXISTS recipe_component_notes (
    id SERIAL PRIMARY KEY,
    recipe_component_id INTEGER NOT NULL REFERENCES recipe_components(id) ON DELETE CASCADE,
    notes TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_recipe_component_notes_rec_com_id ON recipe_component_notes(recipe_component_id);

CREATE TRIGGER update_recipe_component_notes_updated_at
BEFORE UPDATE ON recipe_component_notes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

