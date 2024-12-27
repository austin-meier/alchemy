use std::{fmt::Display, path::{Path, PathBuf}, str::FromStr};
use crate::ingredient::{self, traits::dimension::Rectangle, types::shape::ShapeIngredient, types::shape::StrokeColor};
use crate::ingredient::types::base::Ingredient;
use super::document::AlchemyDocument;


#[derive(Debug)]
pub enum AlchemyPDFError {
	InvalidOutputPath,
}

impl Display for AlchemyPDFError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AlchemyPDFError::InvalidOutputPath => f.write_str("The PDF output path failed to be created")
    }
  }
}


impl AlchemyDocument {
	pub fn to_pdf(&self, path: &Path) -> Result<PathBuf, AlchemyPDFError> {
    //let (doc, page1, layer1) = printpdf::PdfDocument::new();
    println!("{}", self.get_biggest_frame());
		Ok(PathBuf::from_str("../").unwrap())
	}

  pub fn get_cutline_for_page(&self, page: usize) -> Option<&ShapeIngredient> {
    let page_ingredients = self.pages.get(page)?;
    page_ingredients
      .iter()
      .find_map(|ingredient_id| {
        let ingredient = self.ingredients.get(ingredient_id)?;
        match ingredient {
          Ingredient::Shape(shape) => {
            let color = shape.stroke_color.as_ref()?;
            if color.name.to_ascii_lowercase().contains("dieline") {
              return Some(shape);
            } else {
              return None;
            }
          },
          _ => None
        }
      })
    }

  fn get_biggest_frame(&self) -> Rectangle {
    self.ingredients.values().fold(
      Rectangle::new(),
      |acc, ingredient| {
        let frame: Rectangle = ingredient.into();
        if acc.area() <  frame.area(){
          frame.into()
        } else {
          acc
        }
      }
    )
  }
}