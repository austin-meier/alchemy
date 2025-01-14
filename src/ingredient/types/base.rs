use super::{
    image::ImageIngredient, rectangle::RectangleIngredient, shape::ShapeIngredient,
    text::TextIngredient,
};
use crate::ingredient::traits::{
    dimension::{Dimension, DimensionUnit, Dimensionable},
    id::Identifyable,
    position::Positionable,
};

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Ingredient {
    Unknown,

    #[serde(alias = "text")]
    Text(TextIngredient),

    #[serde(alias = "volatileImage")]
    Image(ImageIngredient),

    #[serde(alias = "shape")]
    Shape(ShapeIngredient),

    #[serde(alias = "rectangle", alias = "fill")]
    Rectangle(RectangleIngredient),

    #[serde(alias = "data")]
    Data,
}

#[derive(Debug, serde::Deserialize)]
pub struct IngredientFrame {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl Dimensionable for Ingredient {
    fn height(&self) -> Dimension {
        match self {
            Ingredient::Rectangle(rect) => rect.height(),
            Ingredient::Image(image) => image.height(),
            Ingredient::Shape(shape) => shape.height(),
            Ingredient::Text(text) => text.height(),
            _ => Dimension::new(0.0, DimensionUnit::Inch),
        }
    }

    fn width(&self) -> Dimension {
        match self {
            Ingredient::Rectangle(rect) => rect.width(),
            Ingredient::Image(image) => image.width(),
            Ingredient::Shape(shape) => shape.width(),
            Ingredient::Text(text) => text.width(),
            _ => Dimension::new(0.0, DimensionUnit::Inch),
        }
    }
}

impl Identifyable for Ingredient {
    fn uuid(&self) -> &str {
        match self {
            Ingredient::Rectangle(rect) => rect.uuid(),
            Ingredient::Image(image) => image.uuid(),
            Ingredient::Shape(shape) => shape.uuid(),
            Ingredient::Text(text) => text.uuid(),
            _ => "",
        }
    }
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
