CREATE TABLE IF NOT EXISTS recipes (
    id SERIAL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    description TEXT,
    hidden BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_recipes_user_id ON recipes(user_id);

CREATE TRIGGER update_recipes_updated_at
BEFORE UPDATE ON recipes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

