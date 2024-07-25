use chopping_list::configuration::get_configuration;
use chopping_list::startup::get_connection_pool;
use chopping_list::models::user::{User, CreateUserParams};
use chopping_list::models::recipe;
use chopping_list::models::ingredient;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let db = get_connection_pool(&configuration.database);
    let user = create_random_user(&db).await?.expect("Failed to create a new user");
    let recipe = create_japanese_pancakes(&db, &user.id).await?.expect("Failed to create japanese pancakes");
    let ingredients = [
        "large egg",
        "whole milk",
        "pure vanilla extract",
        "cake flour",
        "baking powder",
        "sugar",
        "water",
        "heavy whipping cream",
        "powdered sugar",
        "strawberries",
        "blueberries",
        "banana",
        "maple syrup"
    ];
    let ingredient_descriptions = [
        "50g without the shell",
        "Great choice for extra fluffy soufflés",
        "100% vanilla",
        "For the fluffiest cakes",
        "100% pure colombian baking powder",
        "Don't be scared! It's only sugar",
        "Bruce Lee's favorite",
        "Same as heavy cream",
        "Also known as confectioners' sugar",
        "Mara de Bois",
        "Tastes like juice",
        "Often used as a measuring tool",
        "Straight from the heart of Canada",
    ];
    let handles: Vec<_> = ingredients.iter()
        .zip(ingredient_descriptions.iter())
        .map(|(ingredient, description)| {
            // Need to clone here because it does not live long enough otherwise
            let db = db.clone();
            let ingredient = ingredient.to_string();
            let description = description.to_string();
            tokio::task::spawn(async move {
                create_ingredient(&db, &ingredient, &Some(description)).await
            })
        })
        .collect();
    // Await all the handles and collect the results
    let ingredient_results = futures::future::join_all(handles).await;
    for result in ingredient_results {
        match result {
            Ok(Ok(Some(ingredient))) => {
                println!("Created ingredient: {:?}", ingredient)
            },
            Ok(Ok(None)) => println!("No ingredient returned."),
            Ok(Err(e)) => eprintln!("Error creating ingredient: {}", e),
            Err(e) => eprintln!("Task error: {:?}", e),
        }
    }

    println!("Hello, Bin!");
    Ok(())
}

pub async fn create_random_user(db: &sqlx::PgPool) -> Result<Option<User>, chopping_list::models::Error> {
    let create_user_params = CreateUserParams::new_with_default_password(fake_email());
    User::create_user(db, &create_user_params).await
}

pub async fn create_japanese_pancakes(db: &sqlx::PgPool, user_id: &uuid::Uuid) -> Result<Option<recipe::Recipe>, chopping_list::models::Error> {
    let create_params = recipe::CreateParams::new(user_id.clone(), String::from("Fluffy Japanese Soufflé Pancakes"))
        .with_description("These Fluffy Japanese Soufflé Pancakes are like eating cottony clouds, but even better with homemade whipped cream and fresh berries!".to_string())
        .with_prep_time(900)
        .with_cook_time(900)
        .with_rest_time(900);

    let egg_name = "large egg".to_string();
    let egg_description = Some("50g without the shell".to_string());
    let egg = create_ingredient(db, &egg_name, &egg_description).await?;

    let milk_name = "whole milk".to_string();
    let milk_description = Some("Great choice for extra fluffy soufflés".to_string());
    let milk = create_ingredient(db, &milk_name, &milk_description).await?;

    let vanilla_name = "pure vanilla extract".to_string();
    let vanilla_description = Some("100% vanilla".to_string());
    let vanilla = create_ingredient(db, &vanilla_name, &vanilla_description).await?;

    let cake_flour_name = "cake flour".to_string();
    let cake_flour_description = Some("for the fluffiest cakes".to_string());
    let cake_flour = create_ingredient(db, &cake_flour_name, &cake_flour_description).await?;

    let baking_powder_name = "baking powder".to_string();
    let baking_powder_description = Some("100% pure colombian baking powder".to_string());
    let baking_powder = create_ingredient(db, &baking_powder_name, &baking_powder_description).await?;

    let sugar_name = "sugar".to_string();
    let sugar_description = Some("Don't use too much!".to_string());
    let sugar = create_ingredient(db, &sugar_name, &sugar_description).await?;

    let water_name = "water".to_string();
    let water_description = Some("Be like water".to_string());
    let water = create_ingredient(db, &water_name, &water_description).await?;

    let heavy_whipping_cream_name = "water".to_string();
    let heavy_whipping_cream_description = Some("Same as heavy cream".to_string());
    let heavy_whipping_cream = create_ingredient(db, &heavy_whipping_cream_name, &heavy_whipping_cream_description).await?;

    let powdered_sugar_name = "powdered sugar".to_string();
    let powdered_sugar_description = Some("Also known as confectioners' sugar".to_string());
    let powdered_sugar = create_ingredient(db, &powdered_sugar_name, &powdered_sugar_description).await?;

    let strawberries_name = "strawberries".to_string();
    let strawberries_description = Some("Mara de Bois".to_string());
    let strawberries = create_ingredient(db, &strawberries_name, &strawberries_description).await?;

    let blueberries_name = "blueberries".to_string();
    let blueberries_description = Some("Tastes like juice".to_string());
    let blueberries = create_ingredient(db, &blueberries_name, &blueberries_description).await?;

    let bananas_name = "bananas".to_string();
    let bananas_description = Some("Sometimes used as a measuring device".to_string());
    let bananas = create_ingredient(db, &bananas_name, &bananas_description).await?;

    let maple_syrup_name = "maple_syrup".to_string();
    let maple_syrup_description = Some("From deep in the heart of Canada".to_string());
    let maple_syrup = create_ingredient(db, &maple_syrup_name, &maple_syrup_description).await?;

    recipe::Recipe::create_recipe(db, &create_params).await
}

pub async fn create_ingredient(db: &sqlx::PgPool, name: &String, description: &Option<String>) -> Result<Option<ingredient::Ingredient>, chopping_list::models::Error> {
    let create_params = ingredient::CreateParams::new(name.clone(), description.clone());
    ingredient::Ingredient::create(db, &create_params).await
}

pub fn fake_email() -> String {
    SafeEmail().fake::<String>()
}

