CREATE TABLE IF NOT EXISTS recipe_instruction_steps (
    id SERIAL PRIMARY KEY,
    recipe_id INTEGER NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    recipe_instruction_id INTEGER NOT NULL REFERENCES recipe_instructions(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipe_steps_recipe_id ON recipe_instruction_steps(recipe_id);
CREATE UNIQUE INDEX idx_recipe_instruction_steps_step_number ON recipe_instruction_steps(recipe_instruction_id, step_number);

CREATE TRIGGER update_recipe_instruction_steps_updated_at
BEFORE UPDATE ON recipe_instruction_steps
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

