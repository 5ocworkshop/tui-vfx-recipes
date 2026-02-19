// <FILE>src/recipe/types.rs</FILE> - <DESC>Core types for unified recipe module (Recipe, effect refs, Phase enum)</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Schema V2.2 standardization</WCTX>
// <CLOG>Fix None type check to use snake_case "none"</CLOG>

use crate::recipe_schema::{RaRecipeConfig, parser::RaJsonRecipeDefinition};
use crate::recipes::RecipeMeta;
use serde_json::Value;
use std::sync::Arc;

/// Unified recipe wrapper providing single entry point for recipe access.
///
/// Wraps RaJsonRecipeDefinition and provides convenient access to metadata
/// and configuration without exposing internal structure.
#[derive(Debug, Clone)]
pub struct Recipe {
    meta: RecipeMeta,
    config: RaRecipeConfig,
    json_cache: Value,
}

impl Recipe {
    /// Create a new Recipe from a RaJsonRecipeDefinition.
    pub fn new(definition: RaJsonRecipeDefinition) -> Self {
        let meta = RecipeMeta {
            id: Arc::from(definition.id.as_str()),
            title: Arc::from(definition.title.as_str()),
            description: Arc::from(definition.description.as_str()),
        };

        // Cache JSON representation for efficient iteration
        let json_cache = serde_json::to_value(&definition.config).unwrap_or(Value::Null);

        Self {
            meta,
            config: definition.config,
            json_cache,
        }
    }

    /// Get reference to the underlying RaRecipeConfig.
    pub fn config(&self) -> &RaRecipeConfig {
        &self.config
    }

    /// Get reference to recipe metadata.
    pub fn metadata(&self) -> &RecipeMeta {
        &self.meta
    }

    /// Get recipe ID as string slice.
    pub fn id(&self) -> &str {
        &self.meta.id
    }

    /// Get recipe title as string slice.
    pub fn title(&self) -> &str {
        &self.meta.title
    }

    /// Iterate over all shaders in the recipe.
    ///
    /// Returns an iterator of ShaderRef items, each containing:
    /// - shader: Reference to the shader JSON value
    /// - location: JSONPath-style location string (e.g., `pipeline.styles[0].enter_effect.shader`)
    /// - phase: Animation phase (Enter, Dwell, Exit, or legacy spatial_shader is Enter)
    ///
    /// # Example
    /// ```no_run
    /// # use tui_vfx_recipes::recipe::Recipe;
    /// # let recipe: Recipe = todo!();
    /// for shader_ref in recipe.shaders() {
    ///     println!("Shader at {}: {:?}", shader_ref.location, shader_ref.shader);
    /// }
    /// ```
    pub fn shaders(&self) -> impl Iterator<Item = ShaderRef<'_>> {
        let mut shaders = Vec::new();

        if let Some(pipeline) = self.json_cache.get("pipeline") {
            // CRITICAL: Use "styles" (plural) not "style" after serialization
            // This is due to #[serde(rename = "styles")] on RaPipelineConfig
            if let Some(styles) = pipeline.get("styles").and_then(|s| s.as_array()) {
                for (i, style) in styles.iter().enumerate() {
                    // Check enter_effect.shader
                    if let Some(enter) = style.get("enter_effect") {
                        if !enter.is_null() {
                            if let Some(shader) = enter.get("shader") {
                                shaders.push(ShaderRef {
                                    shader,
                                    location: format!("pipeline.styles[{}].enter_effect.shader", i),
                                    phase: Phase::Enter,
                                });
                            }
                        }
                    }
                    // Check dwell_effect.shader
                    if let Some(dwell) = style.get("dwell_effect") {
                        if !dwell.is_null() {
                            if let Some(shader) = dwell.get("shader") {
                                shaders.push(ShaderRef {
                                    shader,
                                    location: format!("pipeline.styles[{}].dwell_effect.shader", i),
                                    phase: Phase::Dwell,
                                });
                            }
                        }
                    }
                    // Check exit_effect.shader
                    if let Some(exit) = style.get("exit_effect") {
                        if !exit.is_null() {
                            if let Some(shader) = exit.get("shader") {
                                shaders.push(ShaderRef {
                                    shader,
                                    location: format!("pipeline.styles[{}].exit_effect.shader", i),
                                    phase: Phase::Exit,
                                });
                            }
                        }
                    }
                    // Check legacy spatial_shader field (used by many existing recipes)
                    if let Some(spatial) = style.get("spatial_shader") {
                        if !spatial.is_null() {
                            shaders.push(ShaderRef {
                                shader: spatial,
                                location: format!("pipeline.styles[{}].spatial_shader", i),
                                phase: Phase::Dwell, // Legacy field maps to Dwell phase (usually)
                            });
                        }
                    }
                }
            }
        }

        shaders.into_iter()
    }

    /// Iterate over all masks in the recipe.
    ///
    /// Returns an iterator of MaskRef items, each containing:
    /// - mask: Reference to the mask JSON value
    /// - location: JSONPath-style location string (e.g., `pipeline.mask.enter`)
    /// - phase: Animation phase (Enter or Exit)
    ///
    /// Filters out mask effects with type "None".
    pub fn masks(&self) -> impl Iterator<Item = MaskRef<'_>> {
        let mut masks = Vec::new();

        if let Some(pipeline) = self.json_cache.get("pipeline") {
            if let Some(mask) = pipeline.get("mask") {
                for (phase_name, phase) in [
                    ("enter", Phase::Enter),
                    ("dwell", Phase::Dwell),
                    ("exit", Phase::Exit),
                ] {
                    if let Some(val) = mask.get(phase_name) {
                        let item = if val.is_array() {
                            val.as_array().and_then(|a| a.first())
                        } else {
                            Some(val)
                        };

                        if let Some(m) = item {
                            if m.get("type").and_then(|t| t.as_str()) != Some("none") {
                                masks.push(MaskRef {
                                    mask: m,
                                    location: format!("pipeline.mask.{}", phase_name),
                                    phase,
                                });
                            }
                        }
                    }
                }
            }
        }

        masks.into_iter()
    }

    /// Iterate over all samplers in the recipe.
    ///
    /// Returns an iterator of SamplerRef items, each containing:
    /// - sampler: Reference to the sampler JSON value
    /// - location: JSONPath-style location string (e.g., `pipeline.sampler.enter`)
    /// - phase: Animation phase (Enter, Dwell, or Exit)
    ///
    /// Filters out sampler effects with type "None".
    pub fn samplers(&self) -> impl Iterator<Item = SamplerRef<'_>> {
        let mut samplers = Vec::new();

        if let Some(pipeline) = self.json_cache.get("pipeline") {
            if let Some(sampler) = pipeline.get("sampler") {
                for (phase_name, phase) in [
                    ("enter", Phase::Enter),
                    ("dwell", Phase::Dwell),
                    ("exit", Phase::Exit),
                ] {
                    if let Some(val) = sampler.get(phase_name) {
                        let item = if val.is_array() {
                            val.as_array().and_then(|a| a.first())
                        } else {
                            Some(val)
                        };

                        if let Some(s) = item {
                            if s.get("type").and_then(|t| t.as_str()) != Some("none") {
                                samplers.push(SamplerRef {
                                    sampler: s,
                                    location: format!("pipeline.sampler.{}", phase_name),
                                    phase,
                                });
                            }
                        }
                    }
                }
            }
        }

        samplers.into_iter()
    }

    /// Iterate over all filters in the recipe.
    ///
    /// Returns an iterator of FilterRef items, each containing:
    /// - filter: Reference to the filter JSON value
    /// - location: JSONPath-style location string (e.g., `pipeline.filter.enter`)
    /// - phase: Animation phase (Enter, Dwell, or Exit)
    ///
    /// Filters out filter effects with type "None".
    pub fn filters(&self) -> impl Iterator<Item = FilterRef<'_>> {
        let mut filters = Vec::new();

        if let Some(pipeline) = self.json_cache.get("pipeline") {
            if let Some(filter) = pipeline.get("filter") {
                for (phase_name, phase) in [
                    ("enter", Phase::Enter),
                    ("dwell", Phase::Dwell),
                    ("exit", Phase::Exit),
                ] {
                    if let Some(val) = filter.get(phase_name) {
                        let item = if val.is_array() {
                            val.as_array().and_then(|a| a.first())
                        } else {
                            Some(val)
                        };

                        if let Some(f) = item {
                            if f.get("type").and_then(|t| t.as_str()) != Some("none") {
                                filters.push(FilterRef {
                                    filter: f,
                                    location: format!("pipeline.filter.{}", phase_name),
                                    phase,
                                });
                            }
                        }
                    }
                }
            }
        }

        filters.into_iter()
    }

    /// Iterate over all style layers in the recipe.
    ///
    /// Returns an iterator of StyleLayerRef items, each containing:
    /// - style: Reference to the style layer JSON value
    /// - index: Zero-based index of the style layer
    ///
    /// # Example
    /// ```no_run
    /// # use tui_vfx_recipes::recipe::Recipe;
    /// # let recipe: Recipe = todo!();
    /// for style_ref in recipe.style_layers() {
    ///     println!("Style layer {}: {:?}", style_ref.index, style_ref.style);
    /// }
    /// ```
    pub fn style_layers(&self) -> impl Iterator<Item = StyleLayerRef<'_>> {
        let mut style_layers = Vec::new();

        if let Some(pipeline) = self.json_cache.get("pipeline") {
            // CRITICAL: Use "styles" (plural) not "style" after serialization
            if let Some(styles) = pipeline.get("styles").and_then(|s| s.as_array()) {
                for (i, style) in styles.iter().enumerate() {
                    style_layers.push(StyleLayerRef { style, index: i });
                }
            }
        }

        style_layers.into_iter()
    }
}

/// Reference to a shader effect within a recipe, including location context.
///
/// Location string format: `pipeline.styles\[0\].enter_effect.shader`
/// This enables clear error messages when validation or extraction fails.
#[derive(Debug)]
pub struct ShaderRef<'a> {
    pub shader: &'a Value,
    pub location: String,
    pub phase: Phase,
}

/// Reference to a mask effect within a recipe, including location context.
///
/// Location string format: `pipeline.styles\[0\].enter_effect.mask`
#[derive(Debug)]
pub struct MaskRef<'a> {
    pub mask: &'a Value,
    pub location: String,
    pub phase: Phase,
}

/// Reference to a sampler effect within a recipe, including location context.
///
/// Location string format: `pipeline.styles\[0\].enter_effect.sampler`
#[derive(Debug)]
pub struct SamplerRef<'a> {
    pub sampler: &'a Value,
    pub location: String,
    pub phase: Phase,
}

/// Reference to a filter effect within a recipe, including location context.
///
/// Location string format: `pipeline.styles\[0\].enter_effect.filter`
#[derive(Debug)]
pub struct FilterRef<'a> {
    pub filter: &'a Value,
    pub location: String,
    pub phase: Phase,
}

/// Reference to a style layer within a recipe.
///
/// Location string format: `pipeline.styles\[0\]`
#[derive(Debug)]
pub struct StyleLayerRef<'a> {
    pub style: &'a Value,
    pub index: usize,
}

/// Animation phase enum for effect references.
///
/// Maps to the three animation phases in the pipeline:
/// - Enter: Initial animation as content appears
/// - Dwell: Steady state while content is visible
/// - Exit: Fade-out or slide-out animation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Enter,
    Dwell,
    Exit,
}

impl Phase {
    /// Get phase name as string (lowercase).
    pub fn as_str(&self) -> &'static str {
        match self {
            Phase::Enter => "enter",
            Phase::Dwell => "dwell",
            Phase::Exit => "exit",
        }
    }
}

/// Error types for recipe operations.
///
/// Covers the full recipe lifecycle: loading, parsing, validation, and template resolution.
#[derive(Debug, Clone)]
pub enum RecipeError {
    /// I/O error during file operations (file not found, permission denied, etc.)
    IoError(String),

    /// JSON parsing or deserialization error
    ParseError(String),

    /// Recipe validation error (missing required fields, invalid effect structure, etc.)
    ValidationError(String),

    /// Template resolution error (extends chain cycle, template not found, etc.)
    TemplateError(String),
}

impl std::fmt::Display for RecipeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecipeError::IoError(msg) => write!(f, "I/O error: {}", msg),
            RecipeError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            RecipeError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            RecipeError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl std::error::Error for RecipeError {}

// <FILE>src/recipe/types.rs</FILE> - <DESC>Core types for unified recipe module (Recipe, effect refs, Phase enum)</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
