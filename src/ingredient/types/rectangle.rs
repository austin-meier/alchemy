use crate::ingredient::traits::{dimension::{Dimension, Dimensionable}, id::Identifyable, stroke::StrokeColor};
use crate::ingredient::helpers::{layer::Layer, deserializers::parse_layer};
use super::base::IngredientFrame;


#[derive(Debug, serde::Deserialize)]
pub struct RectangleIngredient {
  pub id: String,

  #[serde(alias = "viewLayer")]
  #[serde(deserialize_with="parse_layer")]
  pub layer: Layer,

  #[serde(alias = "rect")]
  pub frame: IngredientFrame,

  #[serde(alias = "strokeColor")]
  pub stroke_color: Option<StrokeColor>,
}

impl Identifyable for RectangleIngredient {
  fn uuid(&self) -> &str {
    self.id.as_str()
  }
}

impl Dimensionable for RectangleIngredient {
  fn height(&self) -> Dimension {
    self.frame.height()
  }

  fn width(&self) -> Dimension {
    self.frame.width()
  }
}