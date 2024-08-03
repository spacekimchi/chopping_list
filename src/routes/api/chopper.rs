use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Router,
};
use axum::Extension;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use crate::startup::AppState;
use reqwest::Client;
use crate::constants::{
    self,
    route_paths,
};
use std::time::Duration;
use regex::Regex;
use sqlx::PgPool;
use crate::models::ingredient::{Ingredient, CreateIngredientParams};
use crate::models::recipe::{Recipe, CreateRecipeParams};
use crate::models::recipe_component::{RecipeComponent, CreateRecipeComponentParams};
use crate::models::recipe_component_ingredient::{RecipeComponentIngredient, CreateRecipeComponentIngredientParams};
use crate::models::recipe_instruction::{RecipeInstruction, CreateRecipeInstructionParams};
use crate::models::recipe_instruction_step::{RecipeInstructionStep, CreateRecipeInstructionStepParams};
use crate::models::unit::{Unit, CreateUnitParams};
use crate::models::user::User;
use crate::user::AuthSession;

#[derive(Deserialize)]
pub struct ChopperRequest {
    text: String,
    #[serde(rename = "sourceUrl")]
    source_url: String,
    hostname: String,
    pathname: String,
}

#[derive(Serialize)]
pub struct ChopperResponse {
    recipe: Recipe,
}

pub fn routes(state: &AppState) -> Router {
    Router::new()
        .route(route_paths::CHOPPER, post(self::post::chopper))
        .layer(axum::middleware::from_fn_with_state(state.clone(), crate::middleware::api_auth::api_key_auth))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChopperRecipe {
    pub description: String,
    pub name: String,
    pub prep_time: i32,
    pub rest_time: i32,
    pub cook_time: i32,
    pub servings: i32,
    pub source_url: String,
    pub components: Vec<ChopperRecipeComponent>,
    pub instructions: Vec<ChopperRecipeInstruction>,
}

impl ChopperRecipe {
    pub async fn add_to_user(&self, db: &PgPool, user_id: &uuid::Uuid) -> Result<Recipe, crate::models::Error> {
        let create_recipe_params = CreateRecipeParams {
            user_id: user_id.clone(),
            description: self.description.clone(),
            name: self.name.clone(),
            is_public: false,
            prep_time: Some(self.prep_time),
            rest_time: Some(self.rest_time),
            cook_time: Some(self.cook_time),
            servings: Some(self.servings),
            source_url: Some(self.source_url.clone()),
        };
        let recipe = Recipe::create(db, &create_recipe_params).await?.expect("Unable to create recipe");
        for component in &self.components {
            let create_recipe_component_params = CreateRecipeComponentParams {
                recipe_id: recipe.id,
                name: component.name.clone(),
                is_optional: component.is_optional,
            };
            let recipe_component = RecipeComponent::create(db, &create_recipe_component_params).await?.expect("Unable to create recipe component");

            for comp_ing in &component.ingredients {
                let ingredient = match Ingredient::find_by_name(db, comp_ing.name.as_str()).await? {
                    Some(ing) => ing,
                    None => {
                        let ing_params = CreateIngredientParams::new(comp_ing.name.to_string(), comp_ing.description.to_string());
                        Ingredient::create(db, &ing_params).await?
                    }
                };

                let unit = match Unit::find_by_name(db, comp_ing.unit.as_str()).await? {
                    Some(unit) => unit,
                    None => {
                        let unit_params = CreateUnitParams::new(&comp_ing.unit.to_string());
                        Unit::create(db, &unit_params).await?
                    }
                };


                let create_component_ingredient_params = CreateRecipeComponentIngredientParams {
                    recipe_component_id: recipe_component.id,
                    ingredient_id: ingredient.id,
                    unit_id: unit.id,
                    quantity_numerator: comp_ing.quantity_numerator,
                    quantity_denominator: comp_ing.quantity_denominator,
                    is_optional: false,
                };
                RecipeComponentIngredient::create(db, &create_component_ingredient_params).await?.expect("Failed to create recipe component ingredient");
            }
        }

        for instruction in &self.instructions {
            let create_recipe_instruction_params = CreateRecipeInstructionParams {
                recipe_id: recipe.id,
                order_idx: instruction.order_idx,
                title: instruction.title.clone(),
            };
            let recipe_instruction = RecipeInstruction::create(db, &create_recipe_instruction_params).await?.expect("Failed to create recipe instruction");

            for inst_step in &instruction.steps {
                let create_recipe_instruction_step_params = CreateRecipeInstructionStepParams {
                    recipe_id: recipe.id,
                    recipe_instruction_id: recipe_instruction.id,
                    step_number: inst_step.step_number,
                    content: inst_step.content.clone(),
                };

                RecipeInstructionStep::create(db, &create_recipe_instruction_step_params).await?.expect("No recip einstruction step created");
            }
        }

        Ok(recipe)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChopperRecipeComponent {
    pub name: String,
    pub is_optional: bool,
    pub ingredients: Vec<ChopperComponentIngredient>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChopperComponentIngredient {
    pub description: String,
    pub is_optional: bool,
    pub name: String,
    pub quantity_denominator: i32,
    pub quantity_numerator: i32,
    pub unit: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChopperRecipeInstruction {
    pub order_idx: i32,
    pub title: String,
    pub steps: Vec<ChopperInstructionStep>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ChopperInstructionStep {
    pub step_number: i32,
    pub content: String,
}

mod post {
    use super::*;

    #[axum::debug_handler]
    pub async fn chopper(
        Extension(state): Extension<AppState>,
        user: axum::extract::Extension<User>,
        Json(payload): Json<ChopperRequest>,
    ) -> impl IntoResponse {
        let recipe = match parse_recipe_with_openai(&payload, &state).await {
            Ok(recipe) => {
                recipe
            },
            Err(e) => {
                println!("Error in choppa: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e)).into_response()
            }
        };

        match recipe.add_to_user(&state.db, &user.id).await {
            Ok(recipe) => (StatusCode::OK, Json(ChopperResponse { recipe })).into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", err)).into_response()
        }
    }
}

async fn parse_recipe_with_openai(chopper_request: &ChopperRequest, _state: &AppState) -> Result<ChopperRecipe, Box<dyn std::error::Error>> {
    let openai_api_key = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");

    println!("Creating HTTP client...");
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()?;

    println!("Preparing request...");
    let request_body = json!({
        "model": "gpt-4o-mini",
        "messages": [{
            "role": "user",
            "content": format!("{}: {}", constants::strings::RECIPE_PROOMPT, chopper_request.text)
        }],
        "temperature": 0.4
    });

    println!("Request body: {}", serde_json::to_string_pretty(&request_body)?);

    println!("Sending request to OpenAI...");
    let response = client.post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .json(&request_body)
        .send()
        .await?;

    println!("Response status: {:?}", response.status());

    let response_text = response.text().await?;
    println!("Response body: {}", response_text);

    // Parse the JSON response
    let parsed_response: Value = serde_json::from_str(&response_text)?;

    // Extract the content string
    let content = parsed_response["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("Failed to extract content from response")?;

    // Use regex to extract the JSON string from within the backticks, if present
    let re = Regex::new(r"```json\s*([\s\S]*?)\s*```").unwrap();
    let json_str = if let Some(captures) = re.captures(content) {
        captures.get(1).map_or(content, |m| m.as_str())
    } else {
        content
    };

    println!("Received JSON: {}", json_str);

    let recipe_value: serde_json::Value = serde_json::from_str(json_str)?;

    let recipe = ChopperRecipe {
        name: recipe_value["name"].as_str().unwrap_or_default().to_string(),
        description: recipe_value["description"].as_str().unwrap_or_default().to_string(),
        prep_time: recipe_value["prep_time"].as_i64().unwrap_or_default() as i32,
        rest_time: recipe_value["rest_time"].as_i64().unwrap_or_default() as i32,
        cook_time: recipe_value["cook_time"].as_i64().unwrap_or_default() as i32,
        servings: recipe_value["servings"].as_i64().unwrap_or_default() as i32,
        source_url: chopper_request.source_url.clone(),
        components: recipe_value["components"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|comp| ChopperRecipeComponent {
                name: comp["name"].as_str().unwrap_or_default().to_string(),
                is_optional: comp["is_optional"].as_bool().unwrap_or_default(),
                ingredients: comp["ingredients"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|ing| ChopperComponentIngredient {
                        name: ing["name"].as_str().unwrap_or_default().to_string(),
                        description: ing["description"].as_str().unwrap_or_default().to_string(),
                        is_optional: ing["is_optional"].as_bool().unwrap_or_default(),
                        quantity_numerator: ing["quantity_numerator"].as_i64().unwrap_or_default() as i32,
                        quantity_denominator: ing["quantity_denominator"].as_i64().unwrap_or_default() as i32,
                        unit: ing["unit"].as_str().unwrap_or_default().to_string(),
                    })
                    .collect(),
            })
            .collect(),
        instructions: recipe_value["instructions"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .map(|inst| ChopperRecipeInstruction {
                order_idx: inst["order_idx"].as_i64().unwrap_or_default() as i32,
                title: inst["title"].as_str().unwrap_or_default().to_string(),
                steps: inst["steps"]
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .map(|step| ChopperInstructionStep {
                        step_number: step["step_number"].as_i64().unwrap_or_default() as i32,
                        content: step["content"].as_str().unwrap_or_default().to_string(),
                    })
                    .collect(),
            })
            .collect(),
    };

    // Convert the recipe back to a pretty-printed string
    // let recipe_json_string = serde_json::to_string_pretty(&recipe)?;

    Ok(recipe)
}

fn _clean_text(text: &str) -> String {
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(text, " ").into_owned()
}
