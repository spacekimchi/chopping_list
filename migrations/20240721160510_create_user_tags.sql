CREATE TABLE IF NOT EXISTS user_tags (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL references users (id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_user_tags_name ON user_tags(name);

CREATE TRIGGER update_user_tags_updated_at
BEFORE UPDATE ON user_tags
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

