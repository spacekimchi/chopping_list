use chopping_list::configuration::get_configuration;
use chopping_list::startup::get_connection_pool;
use chopping_list::models::user::{User, CreateUserParams};
use chopping_list::models::recipe::{Recipe, CreateRecipeParams};
use chopping_list::models::recipe_instruction_step::{RecipeInstructionStep, CreateRecipeInstructionStepParams};
use chopping_list::models::recipe_instruction::{RecipeInstruction, CreateRecipeInstructionParams};
use chopping_list::models::unit::Unit;
use chopping_list::models::ingredient::{Ingredient, CreateIngredientParams};
use chopping_list::models::recipe_component::{RecipeComponent, CreateRecipeComponentParams};
use chopping_list::models::recipe_component_ingredient::{RecipeComponentIngredient, CreateRecipeComponentIngredientParams};
use chopping_list::models::unit;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;

use sqlx::PgPool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db = get_connection_pool(&configuration.database);
    let user = create_random_user(&db).await?.expect("Failed to create a new user");
    unit::create_default_units(&db).await?;
    let _frijoles = seed_habichuelas_guisadas(&db, &user).await?;
    let _kimchi_jjigae = seed_kimchi_jjigae(&db, &user).await?;

    Ok(())
}

pub async fn create_random_user(db: &sqlx::PgPool) -> Result<Option<User>, chopping_list::models::Error> {
    let create_user_params = CreateUserParams::new_with_default_password(fake_email());
    User::create_user(db, &create_user_params).await
}

async fn seed_habichuelas_guisadas(db: &PgPool, user: &User) -> Result<Recipe, chopping_list::models::Error> {
    // 1. Create the recipe (unchanged)
    let recipe_params = CreateRecipeParams {
        user_id: user.id,
        name: "Habichuelas Guisadas (Puerto Rican Stewed Beans)".to_string(),
        description: Some("Habichuelas Guisadas are a Puerto Rican staple, featuring beans stewed in a tomato-based broth infused with country ham, sofrito, sazón, and Mediterranean herbs. This earthy, complex dish pairs beautifully with rice but can also be enjoyed as a standalone meal.".to_string()),
        is_public: true,
        prep_time: Some(5),
        cook_time: Some(40),
        rest_time: Some(0),
        servings: Some(4),
        source_url: Some("https://jinz.co".to_string()),
    };

    let recipe = Recipe::create(db, &recipe_params).await?.expect("Failed to create recipe");

    // 2. Create recipe component
    let component_params = CreateRecipeComponentParams {
        recipe_id: recipe.id,
        name: "Main Ingredients".to_string(),
        is_optional: false,
    };
    let component = RecipeComponent::create(db, &component_params).await?.expect("Failed to create recipe component");

    // 3. Create ingredients and recipe component ingredients
    let ingredients = vec![
        ("Olive oil", "tablespoon", 1, 1, false),
        ("Country ham", "cup", 1, 4, true),
        ("Puerto Rican sofrito", "cup", 1, 4, false),
        ("Tomato sauce", "cup", 1, 4, false),
        ("Sazón con achiote y culantro", "teaspoon", 3, 2, false),
        ("Ground cumin", "teaspoon", 1, 4, false),
        ("Dried oregano", "teaspoon", 1, 2, false),
        ("Dried bay leaves", "piece", 2, 1, false),
        ("Low sodium chicken broth", "cup", 2, 1, false),
        ("Pink beans (habichuelas rosadas)", "can", 2, 1, false),
        ("Potato", "cup", 1, 3, false),
        ("Pimento-stuffed olives", "piece", 8, 1, false),
        ("Fresh cilantro", "tablespoon", 2, 1, false),
        ("Adobo seasoning", "to_taste", 1, 1, false),
    ];

    for (name, unit_name, quantity_num, quantity_denom, is_optional) in ingredients {
        let ingredient = match Ingredient::find_by_name(db, name).await? {
            Some(ing) => ing,
            None => {
                let ing_params = CreateIngredientParams::new(name.to_string());
                Ingredient::create(db, &ing_params).await?
            }
        };

        let unit = Unit::find_by_name(db, unit_name).await?.expect("Unit not found");

        let recipe_ing_params = CreateRecipeComponentIngredientParams {
            recipe_component_id: component.id,
            ingredient_id: ingredient.id,
            unit_id: unit.id,
            quantity_numerator: quantity_num,
            quantity_denominator: quantity_denom,
            is_optional,
        };

        RecipeComponentIngredient::create(db, &recipe_ing_params).await?;
    }

    // 4. Create recipe instructions (unchanged)
    let instructions = vec![
        CreateRecipeInstructionParams {
            recipe_id: recipe.id,
            order_idx: 1,
            title: "Prepare and Cook".to_string(),
        },
    ];

    for instruction in instructions {
        RecipeInstruction::create(db, &instruction).await?;
    }

    // 5. Create recipe instruction steps (unchanged)
    let steps = vec![
        (1, "In a medium-sized saucepan, heat the olive oil over medium heat. Add the chopped ham and sauté for 2-3 minutes until it starts to caramelize."),
        (1, "Add the sofrito and Sazón seasoning. Sauté for 2 minutes until fragrant."),
        (1, "Add the tomato sauce, oregano, bay leaves, and cumin. Sauté for 1 minute."),
        (1, "Add the chicken stock, beans (with their liquid), chopped potato, pumpkin or carrots, olives, and chopped cilantro. Cover and bring the mixture to a boil."),
        (1, "Once the mixture comes to a boil, reduce to a simmer and cook for 30-40 minutes, stirring occasionally. Allow the flavors to meld, the beans to become tender, and the pumpkin/carrots to cook. The mixture should be creamy, not soupy."),
        (1, "Check for seasoning and add Adobo or salt if needed."),
        (1, "Serve over white or yellow rice, with an extra sprinkle of cilantro if desired."),
    ];

    for (instruction_order_idx, step_content) in steps {
        let instruction = RecipeInstruction::find_by_recipe_id_and_order_idx(db, recipe.id, instruction_order_idx).await?
            .expect("Recipe instruction not found");

        let step_params = CreateRecipeInstructionStepParams {
            recipe_id: recipe.id,
            recipe_instruction_id: instruction.id,
            step_number: RecipeInstructionStep::next_step_number(db, instruction.id).await?,
            content: step_content.to_string(),
        };

        RecipeInstructionStep::create(db, &step_params).await?;
        }

    // 6. Add tags (unchanged)
    let tags = vec!["Puerto Rican", "Beans", "Stew"];
    for tag_name in tags {
        recipe.add_tag(db, tag_name).await?;
        }

    Ok(recipe)
}

async fn seed_kimchi_jjigae(db: &PgPool, user: &User) -> Result<Recipe, chopping_list::models::Error> {
    // 1. Create the recipe (unchanged)
    let recipe_params = CreateRecipeParams {
        user_id: user.id,
        name: "Kimchi Stew (Kimchi Jjigae)".to_string(),
        description: Some("Kimchi Jjigae is a comforting and spicy stew made with kimchi, pork, tofu, and a savory broth. It's perfect for warming up on a cold day and is traditionally enjoyed with a bowl of rice.".to_string()),
        is_public: true,
        prep_time: Some(20),
        cook_time: Some(35),
        rest_time: Some(0),
        servings: Some(4),
        source_url: Some("https://kimchi.jinz.co".to_string())
    };

    let recipe = Recipe::create(db, &recipe_params).await?.expect("Failed to create recipe");

    // 2. Create recipe components
    let components = vec![
        ("For the Kimchi Stew", false),
        ("For the Anchovy Stock", false),
    ];

    for (component_name, is_optional) in components {
        let component_params = CreateRecipeComponentParams {
            recipe_id: recipe.id,
            name: component_name.to_string(),
            is_optional
        };
        RecipeComponent::create(db, &component_params).await?;
    }

    // 3. Create ingredients and recipe component ingredients
    let ingredients = vec![
        ("For the Kimchi Stew", "Kimchi", "pound", 1, 1, false),
        ("For the Kimchi Stew", "Kimchi brine", "cup", 1, 4, false),
        ("For the Kimchi Stew", "Pork shoulder or pork belly", "pound", 1, 2, false),
        ("For the Kimchi Stew", "Tofu", "package", 1, 2, true),
        ("For the Kimchi Stew", "Green onions", "piece", 3, 1, false),
        ("For the Kimchi Stew", "Onion", "piece", 1, 1, false),
        ("For the Kimchi Stew", "Kosher salt", "teaspoon", 1, 1, false),
        ("For the Kimchi Stew", "Sugar", "teaspoon", 2, 1, false),
        ("For the Kimchi Stew", "Gochugaru (Korean hot pepper flakes)", "teaspoon", 2, 1, false),
        ("For the Kimchi Stew", "Gochujang (Korean hot pepper paste)", "tablespoon", 1, 1, false),
        ("For the Kimchi Stew", "Toasted sesame oil", "teaspoon", 1, 1, false),
        ("For the Kimchi Stew", "Anchovy stock (or chicken or beef broth)", "cup", 2, 1, false),
        ("For the Anchovy Stock", "Dried anchovies", "piece", 7, 1, false),
        ("For the Anchovy Stock", "Korean radish (or daikon radish)", "cup", 1, 3, false),
        ("For the Anchovy Stock", "Dried kelp", "piece", 1, 1, false),
        ("For the Anchovy Stock", "Water", "cup", 4, 1, false),
    ];

    for (component_name, name, unit_name, quantity_num, quantity_denom, is_optional) in ingredients {
        let component = RecipeComponent::find_by_recipe_id_and_name(db, recipe.id, &component_name.to_string()).await?
            .expect("Recipe component not found");

        let ingredient = match Ingredient::find_by_name(db, name).await? {
            Some(ing) => ing,
            None => {
                let ing_params = CreateIngredientParams::new(name.to_string());
                Ingredient::create(db, &ing_params).await?
            }
        };

        let unit = Unit::find_by_name(db, unit_name).await?.expect("Unit not found");

        let recipe_ing_params = CreateRecipeComponentIngredientParams {
            recipe_component_id: component.id,
            ingredient_id: ingredient.id,
            unit_id: unit.id,
            quantity_numerator: quantity_num,
            quantity_denominator: quantity_denom,
            is_optional,
        };

        RecipeComponentIngredient::create(db, &recipe_ing_params).await?;
    }

    // 4. Create recipe instructions
    let instructions = vec![
        CreateRecipeInstructionParams {
            recipe_id: recipe.id,
            order_idx: 1,
            title: "Make Anchovy Stock".to_string(),
        },
        CreateRecipeInstructionParams {
            recipe_id: recipe.id,
            order_idx: 2,
            title: "Make Kimchi Stew".to_string(),
        },
    ];

    for instruction in instructions {
        RecipeInstruction::create(db, &instruction).await?;
    }

    // 5. Create recipe instruction steps
    let steps = vec![
        (1, "Combine dried anchovies, radish, green onion roots, and dried kelp in a saucepan."),
        (1, "Add water and bring to a boil over medium-high heat."),
        (1, "Reduce the heat and simmer for 20 minutes."),
        (1, "Lower the heat to low and simmer for another 5 minutes."),
        (1, "Strain the stock and set aside."),
        (2, "Place the kimchi and kimchi brine in a shallow pot. Add the pork and onion slices."),
        (2, "Slice 2 green onions diagonally and add them to the pot."),
        (2, "Add salt, sugar, gochugaru, and gochujang. Drizzle sesame oil over the ingredients."),
        (2, "Pour in the prepared anchovy stock."),
        (2, "Cover and cook for 10 minutes over medium-high heat."),
        (2, "Uncover and stir the stew, mixing in the seasonings. Lay the tofu slices over the top, if using."),
        (2, "Cover and cook for another 10-15 minutes over medium heat."),
        (2, "Chop the remaining green onion and sprinkle over the stew before serving."),
        (2, "Serve hot with rice."),
    ];

    for (instruction_order_idx, step_content) in steps {
        let instruction = RecipeInstruction::find_by_recipe_id_and_order_idx(db, recipe.id, instruction_order_idx).await?
            .expect("Recipe instruction not found");

        let step_params = CreateRecipeInstructionStepParams {
            recipe_id: recipe.id,
            recipe_instruction_id: instruction.id,
            step_number: RecipeInstructionStep::next_step_number(db, instruction.id).await?,
            content: step_content.to_string(),
        };

        RecipeInstructionStep::create(db, &step_params).await?;
        }

    // 6. Add tags
    let tags = vec!["Korean", "Stew", "Spicy", "Kimchi"];
    for tag_name in tags {
        recipe.add_tag(db, tag_name).await?;
        }

    Ok(recipe)
}

pub fn fake_email() -> String {
    SafeEmail().fake::<String>()
}

