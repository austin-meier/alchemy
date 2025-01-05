use crate::ingredient::traits::{dimension::{Dimension, Dimensionable}, id::Identifyable};
use crate::ingredient::helpers::{layer::Layer, deserializers::parse_layer};
use super::base::IngredientFrame;

#[derive(Debug, serde::Deserialize)]
pub struct TextIngredient {
  pub id: String,

  #[serde(alias = "viewLayer")]
  #[serde(deserialize_with="parse_layer")]
  pub layer: Layer,

  #[serde(alias = "rect")]
  pub frame: IngredientFrame
}

impl Identifyable for TextIngredient {
  fn uuid(&self) -> &str {
    self.id.as_str()
  }
}

impl Dimensionable for TextIngredient {
  fn height(&self) -> Dimension {
    self.frame.height()
  }

  fn width(&self) -> Dimension {
    self.frame.width()
  }
}