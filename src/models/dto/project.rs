use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct NewProject {
    pub token: String,
    pub category: String,
    pub contract_address: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProject {
    pub token: Option<String>,
    pub category: Option<String>,
    pub contract_address: Option<String>,
    pub num_chains: Option<i32>,
    pub core_developers: Option<i32>,
    pub code_commits: Option<i32>,
    pub total_value_locked: Option<f64>,
    pub token_max_supply: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectResponse {
    pub id: i32,
    pub token: String,
    pub category: String,
    pub contract_address: Option<String>,
    pub num_chains: Option<i32>,
    pub core_developers: Option<i32>,
    pub code_commits: Option<i32>,
    pub total_value_locked: Option<f64>,
    pub token_max_supply: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}
