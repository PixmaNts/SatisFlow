use crate::models::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ItemsFile {
    pub items: Vec<Item>,
}

#[derive(Deserialize)]
pub struct RecipesFile {
    pub recipes: Vec<Recipe>,
}

#[cfg(target_arch = "wasm32")]
pub async fn load_items() -> Result<HashMap<String, Item>, Box<dyn std::error::Error>> {
    let response = gloo_net::http::Request::get("data/items.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch items.json: {}", e))?;
    
    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read items.json content: {}", e))?;
    
    let items_file: ItemsFile = serde_json::from_str(&content)?;
    
    let mut items = HashMap::new();
    for item in items_file.items {
        items.insert(item.name.clone(), item);
    }
    
    Ok(items)
}

#[cfg(target_arch = "wasm32")]
pub async fn load_recipes() -> Result<HashMap<String, Recipe>, Box<dyn std::error::Error>> {
    let response = gloo_net::http::Request::get("data/recipes.json")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch recipes.json: {}", e))?;
    
    let content = response
        .text()
        .await
        .map_err(|e| format!("Failed to read recipes.json content: {}", e))?;
    
    let recipes_file: RecipesFile = serde_json::from_str(&content)?;
    
    let mut recipes = HashMap::new();
    for recipe in recipes_file.recipes {
        recipes.insert(recipe.name.clone(), recipe);
    }
    
    Ok(recipes)
}

// Desktop/non-WASM fallback
#[cfg(not(target_arch = "wasm32"))]
pub fn load_items() -> Result<HashMap<String, Item>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/items.json")?;
    let items_file: ItemsFile = serde_json::from_str(&content)?;
    
    let mut items = HashMap::new();
    for item in items_file.items {
        items.insert(item.name.clone(), item);
    }
    
    Ok(items)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn load_recipes() -> Result<HashMap<String, Recipe>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string("data/recipes.json")?;
    let recipes_file: RecipesFile = serde_json::from_str(&content)?;
    
    let mut recipes = HashMap::new();
    for recipe in recipes_file.recipes {
        recipes.insert(recipe.name.clone(), recipe);
    }
    
    Ok(recipes)
}