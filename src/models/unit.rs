use serde::{Serialize, Deserialize};
use sqlx::{PgPool, FromRow};
use std::str::FromStr;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Unit {
    pub id: i32,
    pub name: String,
    pub abbreviation: Option<String>,
    pub system: UnitSystem,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[derive(sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum UnitSystem {
    Metric,
    Imperial,
    Universal,
}

impl FromStr for UnitSystem {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "metric" => Ok(UnitSystem::Metric),
            "imperial" => Ok(UnitSystem::Imperial),
            "universal" => Ok(UnitSystem::Universal),
            _ => Err(format!("Invalid unit system: {}", s)),
        }
    }
}

impl std::fmt::Display for UnitSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnitSystem::Metric => write!(f, "metric"),
            UnitSystem::Imperial => write!(f, "imperial"),
            UnitSystem::Universal => write!(f, "universal"),
        }
    }
}

pub struct CreateUnitParams {
    pub name: String,
    pub abbreviation: Option<String>,
    pub system: UnitSystem,
}

impl CreateUnitParams {
    pub fn new(name: &String) -> Self {
        Self {
            name: name.clone(),
            abbreviation: None,
            system: UnitSystem::Universal,
        }
    }
}

impl Unit {
    pub async fn create(db: &PgPool, params: &CreateUnitParams) -> Result<Self, crate::models::Error> {
        let unit = sqlx::query_as(
            "INSERT INTO units (name, abbreviation, system) VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(&params.name)
        .bind(&params.abbreviation)
        .bind(params.system.to_string())
        .fetch_one(db)
        .await?;

        Ok(unit)
    }

    pub async fn find_by_id(db: &PgPool, id: i32) -> Result<Option<Self>, crate::models::Error> {
        let unit = sqlx::query_as("SELECT * FROM units WHERE id = $1")
            .bind(id)
            .fetch_optional(db)
            .await?;

        Ok(unit)
    }

    pub async fn find_by_name(db: &PgPool, name: &str) -> Result<Option<Self>, crate::models::Error> {
        let unit = sqlx::query_as("SELECT * FROM units WHERE name = $1")
            .bind(name)
            .fetch_optional(db)
            .await?;

        Ok(unit)
    }
}

pub async fn create_default_units(db: &PgPool) -> Result<(), crate::models::Error> {
    let default_units = vec![
        CreateUnitParams {
            name: "gram".to_string(),
            abbreviation: Some("g".to_string()),
            system: UnitSystem::Metric,
        },
        CreateUnitParams {
            name: "kilogram".to_string(),
            abbreviation: Some("kg".to_string()),
            system: UnitSystem::Metric,
        },
        CreateUnitParams {
            name: "milliliter".to_string(),
            abbreviation: Some("ml".to_string()),
            system: UnitSystem::Metric,
        },
        CreateUnitParams {
            name: "liter".to_string(),
            abbreviation: Some("L".to_string()),
            system: UnitSystem::Metric,
        },
        CreateUnitParams {
            name: "teaspoon".to_string(),
            abbreviation: Some("tsp".to_string()),
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "tablespoon".to_string(),
            abbreviation: Some("tbsp".to_string()),
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "cup".to_string(),
            abbreviation: Some("cup".to_string()),
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "ounce".to_string(),
            abbreviation: Some("oz".to_string()),
            system: UnitSystem::Imperial,
        },
        CreateUnitParams {
            name: "pound".to_string(),
            abbreviation: Some("lb".to_string()),
            system: UnitSystem::Imperial,
        },
        CreateUnitParams {
            name: "piece".to_string(),
            abbreviation: Some("pc".to_string()),
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "pinch".to_string(),
            abbreviation: None,
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "can".to_string(),
            abbreviation: None,
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "to_taste".to_string(),
            abbreviation: None,
            system: UnitSystem::Universal,
        },
        CreateUnitParams {
            name: "package".to_string(),
            abbreviation: None,
            system: UnitSystem::Universal,
        },
    ];

    for unit_params in default_units {
        if Unit::find_by_name(db, &unit_params.name).await?.is_none() {
            Unit::create(db, &unit_params).await?;
        }
    }

    Ok(())
}
