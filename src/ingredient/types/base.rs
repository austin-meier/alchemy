use serde::{Deserialize, Deserializer};

use crate::ingredient::traits::{dimension::{Dimension, DimensionUnit, Dimensionable}, position::Positionable};

use super::{image::ImageIngredient, text::TextIngredient, shape::ShapeIngredient};

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Ingredient {

    #[serde(alias = "test")]
    Unknown,

    #[serde(alias = "text")]
    Text(TextIngredient),

    #[serde(alias = "volatileImage")]
    Image(ImageIngredient),

    #[serde(alias = "shape", alias = "fill", alias = "rectangle")]
    Shape(ShapeIngredient),

    #[serde(alias = "data")]
    Data
}

#[derive(Debug)]
pub enum Layer {
    None,
    Print,
    Mask,
    Bleed,
    Background,
}

#[derive(Debug, serde::Deserialize)]
pub struct IngredientFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Dimensionable for IngredientFrame {
  fn height(&self) -> Dimension {
    Dimension::new(self.height, DimensionUnit::Inch)
  }

  fn width(&self) -> Dimension {
    Dimension::new(self.width, DimensionUnit::Inch)
  }
}

impl Positionable for IngredientFrame {
    fn get_x(&self) -> Dimension {
        Dimension::new(self.x, DimensionUnit::Inch)
    }

    fn get_y(&self) -> Dimension {
        Dimension::new(self.y, DimensionUnit::Inch)
    }
}

pub fn parse_layer<'de, D>(d: D) -> Result<Layer, D::Error> where D: Deserializer<'de> {
    Deserialize::deserialize(d)
        .map(|layer: Option<_>| {
            let layer_name = layer.unwrap_or("none");
            match layer_name {
                "background" => Layer::Background,
                "bleed" => Layer::Bleed,
                "print" => Layer::Print,
                "mask" => Layer::Mask,
                _ => Layer::None
            }
        })
}