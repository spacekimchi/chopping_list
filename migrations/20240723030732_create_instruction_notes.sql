CREATE TABLE IF NOT EXISTS instruction_notes (
    id SERIAL PRIMARY KEY,
    instruction_id INTEGER NOT NULL REFERENCES instructions(id) ON DELETE CASCADE,
    notes TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_instruction_notes_inst_id ON instruction_notes(instruction_id);

CREATE TRIGGER update_instruction_notes_updated_at
BEFORE UPDATE ON instruction_notes
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

