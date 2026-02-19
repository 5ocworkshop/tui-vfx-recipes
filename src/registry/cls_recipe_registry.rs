// <FILE>src/registry/cls_recipe_registry.rs</FILE> - <DESC>RecipeRegistry implementation</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG6: Recipe String Ownership Refactor</WCTX>
// <CLOG>BREAKING: Updated to use Arc<str>-based RecipeMeta. Fixed comparisons using .as_ref() for proper string comparison semantics</CLOG>

use super::dyn_recipe::DynRecipe;
use crate::recipes::types::RecipeMeta;
#[derive(Default)]
pub struct RecipeRegistry {
    recipes: Vec<Box<dyn DynRecipe + Send + Sync>>,
}
impl RecipeRegistry {
    pub fn new() -> Self {
        Self { recipes: vec![] }
    }
    pub fn register(&mut self, recipe: Box<dyn DynRecipe + Send + Sync>) {
        self.recipes.push(recipe);
    }
    pub fn list(&self) -> Vec<RecipeMeta> {
        let mut items: Vec<_> = self.recipes.iter().map(|r| r.meta()).collect();
        items.sort_by(|a, b| a.id.as_ref().cmp(b.id.as_ref()));
        items
    }
    /// Looks up a recipe by ID. Accepts any string reference.
    pub fn get(&self, id: &str) -> Option<&(dyn DynRecipe + Send + Sync)> {
        self.recipes
            .iter()
            .map(|b| b.as_ref())
            .find(|r| r.meta().id.as_ref() == id)
    }
}

// <FILE>src/registry/cls_recipe_registry.rs</FILE> - <DESC>RecipeRegistry implementation</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
