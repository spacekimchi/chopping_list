-- This table is because a recipe can be an ingredient of another recipe
CREATE TABLE IF NOT EXISTS recipe_components (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    component_recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    instruction_id INTEGER REFERENCES instructions (id) ON DELETE CASCADE,
    quantity INTEGER NOT NULL,
    unit TEXT NOT NULL,
    optional BOOLEAN DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_components_recipe_id ON recipe_components(recipe_id);
CREATE INDEX idx_recipe_components_comp_rec_id ON recipe_components(component_recipe_id);
CREATE INDEX idx_recipe_components_instruction_id ON recipe_components(instruction_id);
CREATE UNIQUE INDEX idx_recipe_components_rec_comp_rec ON recipe_components(recipe_id, component_recipe_id);

CREATE TRIGGER update_recipe_components_updated_at
BEFORE UPDATE ON recipe_components
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

