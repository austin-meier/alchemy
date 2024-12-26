use super::{image::ImageIngredient, text::TextIngredient, shape::ShapeIngredient};
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor};
use url::Url;
use std::str::FromStr;

#[derive(Debug)]
pub enum Ingredient {
    Unknown,
    Text(TextIngredient),
    Image(ImageIngredient),
    Shape(ShapeIngredient)
}

impl<'de> Deserialize<'de> for Ingredient {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(IngredientVisitor)
    }
}

pub enum IngredientType {
    Unknown(String),
    Text,
    Image,
    Shape,
}

impl IngredientType {
    fn from_str(str: String) -> Self {
        match str.as_str() {
            "volatileImage" => IngredientType::Image,
            "text" => IngredientType::Text,
            "shape" => IngredientType::Shape,
            _ => IngredientType::Unknown(str.into())
        }
    }
}


/* The Serde Visitor used when parsing the alchemy document */
pub struct IngredientVisitor;

impl<'de> Visitor<'de> for IngredientVisitor {
    type Value = Ingredient;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an object representing an alchemy ingredient (Image, Text, or Shape)")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {

        let mut ingredient_type = IngredientType::Unknown("unknown".into());

        while let Some(key) = map.next_key::<String>()? {
            println!("Key: {}", key);
            match key.as_str() {
                "type" => {
                    ingredient_type = IngredientType::from_str(map.next_value()?);
                },
                _ => {
                    println!("Unknown key: {}", key);
                }
            }
        }

        match ingredient_type {
            IngredientType::Image => {
                Ok(Ingredient::Image(ImageIngredient{src: Url::from_str("http://www.google.com").unwrap()}))
            },

            IngredientType::Shape => {
                Ok(Ingredient::Shape(ShapeIngredient{test: 1}))
            },

            IngredientType::Text => {
                Ok(Ingredient::Text(TextIngredient{font: "test".into()}))
            },

            IngredientType::Unknown(_type_found) => {
                Ok(Ingredient::Unknown)
            }
        }

    }
}