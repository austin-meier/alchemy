use std::{cmp::Ordering, collections::HashMap};
use crate::ingredient::{traits::dimension::{Dimensionable, Rectangle}, types::base::Ingredient};

#[derive(serde::Deserialize, Debug)]
pub struct AlchemyDocument {
	/* dpi is the ratio alchemy is using for convert pixel values to measurable IRL dimensions */
	pub dpi: u32,

	/* Pages is a 2d array of pages in order of appearance. Each page vector is an
       array of uuid ingredient identifies in their respective order*/
  pub pages: Vec<Vec<String>>,

	pub ingredients: HashMap<String, Ingredient>
}

impl AlchemyDocument {
	pub fn get_dielines_for_page(&self, page: usize) -> Option<Vec<&Ingredient>> {
    let page_ingredients = self.pages.get(page)?;
    let ingredients = page_ingredients
      .iter()
      .filter_map(|ingredient_id| {
        let ingredient = self.ingredients.get(ingredient_id)?;
				/* TODO: Figure out how to match this a little more elegantly */
        match ingredient {
					Ingredient::Rectangle(shape) => {
            let color = shape.stroke_color.as_ref()?;
            if color.name.to_ascii_lowercase().contains("dieline") {
              return Some(ingredient);
            } else {
              return None;
            }
					},
          Ingredient::Shape(shape) => {
            let color = shape.stroke_color.as_ref()?;
            if color.name.to_ascii_lowercase().contains("dieline") {
              return Some(ingredient);
            } else {
              return None;
            }
          },
          _ => None
        }
      }).collect();
      Some(ingredients)
    }

	pub fn get_ingredients_on_page(&self, page_num: usize) -> Vec<&Ingredient> {
		self.pages
			.get(page_num)
			.unwrap_or(&Vec::new())
			.iter()
			.filter_map(|ingredient_id| self.ingredients.get(ingredient_id))
			.collect()
	}

  pub fn get_biggest_ingredient(&self) -> Option<&Ingredient> {
		self.ingredients.values().max_by(|a, b| {
			a.area().partial_cmp(&b.area()).unwrap_or(Ordering::Less)
		})
	}

}