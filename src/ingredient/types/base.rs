use super::{image::ImageIngredient, text::TextIngredient, shape::ShapeIngredient};

#[derive(serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Ingredient {
    Unknown,

    #[serde(alias = "text")]
    Text(TextIngredient),

    #[serde(alias = "volatileImage")]
    Image(ImageIngredient),

    #[serde(alias = "shape")]
    Shape(ShapeIngredient)
}