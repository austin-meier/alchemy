use crate::ingredient::traits::{dimension::{Dimension, DimensionUnit, Dimensionable}, id::Identifyable};
use super::base::{IngredientFrame, Layer, parse_layer};

#[derive(Debug, serde::Deserialize)]
pub struct ImageIngredient {
  pub id: String,

  #[serde(alias = "viewLayer")]
  #[serde(deserialize_with="parse_layer")]
  pub layer: Layer,

  #[serde(alias = "rect")]
  pub frame: IngredientFrame
}

impl Identifyable for ImageIngredient {
  fn uuid(&self) -> &str {
    self.id.as_str()
  }
}

impl Dimensionable for ImageIngredient {
  fn height(&self) -> Dimension {
    Dimension::new(self.frame.height, DimensionUnit::Inch)
  }

  fn width(&self) -> Dimension {
    Dimension::new(self.frame.width, DimensionUnit::Inch)
  }
}