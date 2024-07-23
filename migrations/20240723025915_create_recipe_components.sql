-- This table is because a recipe can be an ingredient of another recipe
CREATE TABLE IF NOT EXISTS recipe_components (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    component_recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    instruction_id INTEGER REFERENCES instruction (id) ON DELETE CASCADE,
    quantity INTEGER,
    unit TEXT,
    optional DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_components_recipe_id ON recipe_components(recipe_id);
CREATE INDEX idx_recipe_components_ingredient_id ON recipe_components(ingredient_id);
CREATE INDEX idx_recipe_components_instruction_id ON recipe_components(instruction_id);
CREATE UNIQUE INDEX idx_recipe_components_rec_ing_ins ON recipe_components(recipe_id, ingredient_id, instruction_id);

CREATE TRIGGER update_recipe_components_updated_at
BEFORE UPDATE ON recipe_components
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

