CREATE TABLE IF NOT EXISTS recipe_component_ingredients (
    id SERIAL PRIMARY KEY,
    recipe_component_id INTEGER NOT NULL REFERENCES recipe_components(id) ON DELETE CASCADE,
    ingredient_id INTEGER NOT NULL REFERENCES ingredients(id) ON DELETE CASCADE,
    unit_id INTEGER NOT NULL REFERENCES units(id) ON DELETE CASCADE,
    quantity_numerator INTEGER NOT NULL,
    quantity_denominator INTEGER NOT NULL,
    is_optional BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_component_ingredients_component_id ON recipe_component_ingredients(recipe_component_id);
CREATE INDEX idx_recipe_component_ingredients_ingredient_id ON recipe_component_ingredients(ingredient_id);

CREATE TRIGGER update_recipe_component_ingredients_updated_at
BEFORE UPDATE ON recipe_component_ingredients
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

