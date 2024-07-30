CREATE TABLE user_ingredient_tags (
    recipe_id INTEGER REFERENCES recipes(id),
    user_tag_id INTEGER REFERENCES user_tags(id),
    PRIMARY KEY (recipe_id, user_tag_id)
);

