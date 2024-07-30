-- Insert "admin" user.
-- Password is 'hunter42'
INSERT INTO users (id, email, username, password_hash)
VALUES (gen_random_uuid(), 'admin', 'admin', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

INSERT INTO users (id, email, username,  password_hash)
VALUES (gen_random_uuid(), 'basic1@foo.com', 'basic1', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

INSERT INTO users (id, email, username, password_hash)
VALUES (gen_random_uuid(), 'basic2@foo.com', 'basic2', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

INSERT INTO users (id, email, username, password_hash)
VALUES (gen_random_uuid(), 'basic3@foo.com', 'basic3', '$argon2id$v=19$m=19456,t=2,p=1$VE0e3g7DalWHgDwou3nuRA$uC6TER156UQpk0lNQ5+jHM0l5poVjPA1he/Tyn9J4Zw');

-- insert admin role
INSERT INTO roles (name) VALUES ('admin');
-- insert basic role
INSERT INTO roles (name) VALUES ('basic');

-- Set the default user as the admin user in the app
DO $$
DECLARE
    first_user_id UUID;
    admin_role_id INTEGER;
    basic_role_id INTEGER;
    user RECORD;
BEGIN
    -- Retrieve the ID of the first user.
    SELECT id INTO first_user_id FROM users ORDER BY created_at LIMIT 1;

    -- Retrieve the ID of the 'admin' role.
    SELECT id INTO admin_role_id FROM roles WHERE name = 'admin';

    -- Assign 'admin' role to first user.
    INSERT INTO user_roles (user_id, role_id)
    VALUES (first_user_id, admin_role_id);

    -- Retrieve the ID of the 'basic' role.
    SELECT id INTO basic_role_id FROM roles WHERE name = 'basic';

    -- Assign 'basic' role to users without a role.
    INSERT INTO user_roles (user_id, role_id)
    SELECT u.id, basic_role_id
    FROM users u
    LEFT JOIN user_roles ur ON u.id = ur.user_id
    WHERE ur.user_id IS NULL;
END $$;
