use std::collections::HashMap;

use crate::ingredient::types::base::Ingredient;

#[derive(serde::Deserialize, Debug)]
pub struct AlchemyDocument {
	/* dpi is the ratio alchemy is using for convert pixel values to measurable IRL dimensions */
	pub dpi: u32,

	/* Pages is a 2d array of pages in order of appearance. Each page vector is an
       array of uuid ingredient identifies in their respective order*/
  pub pages: Vec<Vec<String>>,

	pub ingredients: HashMap<String, Ingredient>
}