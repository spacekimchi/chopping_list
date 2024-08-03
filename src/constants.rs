//! src/constants.rs
//! This file defines all the constants used throughout the application.
//! Define things here to make life easy
//!

/// Template constants
pub mod html_templates {
    pub const REGISTER: &str = "register.html";
    pub const LOGIN: &str = "login.html";
    pub const HOMEPAGE: &str = "homepage.html";
    pub const E500: &str = "500.html";
    pub const RECIPES_INDEX: &str = "recipes/index.html";
    pub const RECIPES_SHOW: &str = "recipes/show.html";
}

/// email templates
pub mod email_templates {
    pub const EMAIL_VERIFICATION: &str = "emails/email_verification.html";
}

/// Strings
pub mod strings {
    pub const WELCOME_EMAIL_SUBJECT: &str = "Welcome to Chopping List";
    pub const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";
    pub const REGISTER_ACCOUNT_SUCCESS: &str = "Successfully registered account!";
    pub const INVALID_CREDENTIALS: &str = "Invalid Credentials";
    pub const FAILED_TO_COMPILE_SCSS: &str = "Failed to compile SCSS";
    pub const FAILED_TO_WRITE_SCSS: &str = "Failed to write SCSS";
    pub const RECIPE_PROOMPT: &str = "Parse this recipe and tell me the name, a description, prep time, cook time, rest time, servings (if servings isn't, make an educated guess).
prep_time, cook_time, and rest_time should be an integer for how many minutes it takes. Servings should be an integer. If a recipe gives a range for servings, just average and round it.
Also tell me the ingredients for the recipe. I want to know the recipe_component, and then list the ingredients under it. From the ingredients give me the name, a short description of the ingredient, the quantity numerator, quantity denominator, the unit, and if it is optional
Also tell me the recipe instructions. Please provide the instructions with two levels of detail. The first level should be the title of the instruction, and the second level should be the steps for that instruction. Step numbers should be the order of steps for that instruction.
Please give those to me in JSON in this exact format.
{
  name: string,
  description: string,
  prep_time: i32,
  cook_time: i32,
  rest_time: i32,
  servings: i32,
  components: [{
    name: string,
    is_optional: boolean,
    ingredients: [{
      name: string,
      description: string,
      unit: string,
      quantity_numerator: i32,
      quantity_denominator: i32,
      is_optional: bool,
    }]
  }],
  instructions: [{
    order_idx: i32,
    title: string,
    steps: [{
      step_number: i32,
      content: string
    }]
  }]
}";
}

/// paths
pub mod route_paths {
    pub const ROOT: &str = "/";
    pub const REGISTER: &str = "/register";
    pub const LOGIN: &str = "/login";
    pub const LOGOUT: &str = "/logout";
    pub const HEALTH: &str = "/health";
    pub const PROTECTED: &str = "/protected";
    pub const RECIPES: &str = "/recipes";
    pub const API: &str = "/api";
    pub const CHOPPER: &str = "/chopper";
}

