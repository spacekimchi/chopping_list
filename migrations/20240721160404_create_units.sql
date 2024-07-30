CREATE TABLE IF NOT EXISTS units (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    abbreviation TEXT,
    system TEXT NOT NULL DEFAULT 'universal' CHECK (system IN ('metric', 'imperial', 'universal')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_units_updated_at
BEFORE UPDATE ON units
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

