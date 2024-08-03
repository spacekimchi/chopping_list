# Chopping List

### Purpose

Keep share, record, download, and edit all your favorite recipes on your Chopping List!

### Rust

The recommended way of [installing Rust](https://www.rust-lang.org/tools/install) is through rustup

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Backend

The backend is built using [Axum](https://github.com/tokio-rs/axum).

I chose Rust and Axum over other languages and frameworks because of how powerful it is even on a cheap machine.

Rust is also great for web programming because it requires that errors and all cases be handled. The only bugs really would be logic ones.

### PostgreSQL

The project is using [PostgreSQL](https://www.postgresql.org/). You can install PostgreSQL for whatever machine you are using.

There is a docker script for starting a PostgreSQL database inside `scripts/init_db.sh`

Use this command to connect to the PostgreSQL docker container.

`psql -h 127.0.0.1 -p 5432 -U postgres`

### sqlx

This project uses [sqlx-cli](https://github.com/launchbadge/sqlx/tree/main/sqlx-cli) to manage the database. The following command will install for only postgres

`cargo install sqlx-cli --no-default-features --features native-tls,postgres`

We can initialize the database using. Make sure there is a `DATABASE_URL` environment variable set in the `.env` file.

`sqlx database setup`

Add a migration using sqlx

`sqlx migrate add <migration_name>`

Run the migrations with

`sqlx migrate run`

Revert the last migration with

`sqlx migrate revert`

## When deploying to server

Remember to get a copy of the `configuration/local.yaml`, `configuration/base.yaml`, and `configuration/production.yaml`.

`base_url` needs to be set in the `configuration/production.yaml`. This can be set to the domain the project will be hosted on.

Create a systemd service to run the application.

The systemd service loads environment variables using a path. Be sure to restrict reading access to this file in order to protect secrets

## Development

For autocompiling on code changes install cargo-watch with: `cargo install cargo-watch`

Then run `cargo watch -x run`

Run with `cargo watch --no-vcs-ignores -x run` to ignore the .gitignore and use the `.ignore_file` instead

## Emailing

The project is set up to email using SMTP servers.

I recommend using [MailHog](https://github.com/mailhog/MailHog) for testing email sending during development.
You are able to view sent mail at [http://localhost:8025](http://localhost:8025)

The default port for mailhog is 1025, and the default email view for the browser is 8025.

Email templates are placed under the `templates/emails` directory.

## Frontend

Instead of using a frontend framework, this project will use SSR to serve HTML, SCSS, and JavaScript.

The project uses [tera](https://github.com/Keats/tera) for templating. Tera uses the `templates/` directory as a base directory

The scss will be compiled when the project starts using the [grass crate](https://github.com/connorskees/grass)

The project is set up to compile any scss files in the `scss/` directory.

This way, we only need to build the project with cargo. No javascript build dependencies involved!

Instead, we can use vanilla Javascript and HTMX to get a SPA feeling.

Add javascript files to /static/js/ directory and include them in the html wherever they are needed

## Creating Full Recipes

In order to create recipes, we need to have these data:

For Recipe
 - name, description, prep_time, cook_time, rest_time, servings, source_url

For RecipeComponent
 - name, is_optional

For RecipeComponentIngredient
 - name, description, unit, quantity_numerator, quantity_denominator, is_optional

For RecipeInstruction
 - order_idx (instruction order), title

For RecipeInstructionStep
 - step_number, content

A good prompt to ask the AI is something like
```
Parse this recipe and tell me the name, a description, prep time, cook time, rest time, servings (if servings isn't, make an educated guess).
Also tell me the ingredients for the recipe. I want to know the component, and then list the ingredients under it. From the ingredients give me the name, a short description of the ingredient, the quantity numerator, quantity denominator, the unit, and if it is optional
Also tell me the recipe instructions. Please provide the instructions with two levels of detail. The first level should be the title of the instruction, and the second level should be the steps for that instruction.
Give me the recipe, recipe component, and recipe instructions in json format and nothing else. This is the JSON format i want:
{
  name: string,
  description: string,
  prep_time: i32,
  cook_time: i32,
  rest_time: i32,
  servings: i32,
  recipe_component: {
    name: string,
    is_optional: boolean,
    component_ingredients: [{
      name: string,
      description: string,
      unit: string,
      quantity_numerator: i32,
      quantity_denominator: i32,
      is_optional: bool,
    }]
  },
  recipe_instructions: {
    order_idx: i32,
    title: string,
    instruction_step: [{
      step_number: i32,
      content: i32
    }]
  }
}
And finally, please include any tips and tricks.
```

## Tests

Run tests with the command `cargo test`

## Test debugging

[about:debugging](https://firefox-source-docs.mozilla.org/devtools-user/about_colon_debugging/index.html)
[about:debugging](about:debugging)

If you want to run a certain test, you can specify the name of the test.
 - Ex: `cargo test authorized_user_creation` will run tests with names that match `authorized_user_creation`
   - Ex: `authorized_user_creation` and `unauthorized_user_creation` both match `authorized_user_creation`

If you want to capture `println!()` statements when running tests, add `-- --nocapture` to the command.
 - Ex: `cargo test -- --nocapture`

