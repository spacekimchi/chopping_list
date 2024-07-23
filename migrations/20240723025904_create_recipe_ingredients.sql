CREATE TABLE IF NOT EXISTS recipe_ingredients (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes (id) ON DELETE CASCADE,
    ingredient_id INTEGER NOT NULL REFERENCES ingredients (id) ON DELETE CASCADE,
    instruction_id INTEGER REFERENCES instruction (id) ON DELETE CASCADE,
    quantity INTEGER,
    unit TEXT,
    optional DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_ingredients_recipe_id ON recipe_ingredients(recipe_id);
CREATE INDEX idx_recipe_ingredients_ingredient_id ON recipe_ingredients(ingredient_id);
CREATE INDEX idx_recipe_ingredients_instruction_id ON recipe_ingredients(instruction_id);
CREATE UNIQUE INDEX idx_recipe_ingredients_rec_ing_ins ON recipe_ingredients(recipe_id, ingredient_id, instruction_id);

CREATE TRIGGER update_recipe_ingredients_updated_at
BEFORE UPDATE ON recipe_ingredients
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

